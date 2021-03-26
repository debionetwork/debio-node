#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use super::*;


    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    // NOTE: if the visibility of trait store is private but you want to make it available
    // in super, then use `pub(super)` or `pub(crate)` to make it available in crate.
    pub struct Pallet<T>(_);
    // pub struct Pallet<T, I = ()>(PhantomData<T>); // for instantiable pallet
    
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    }

    // Storage --------
    #[pallet::storage]
    #[pallet::getter(fn roles_by_id)]
    pub type UserRoles<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, Vec<Role>>;


    // Dispatchable Calls -- 
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn assign_role(origin: OriginFor<T>, role: super::Role) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match UserRoles::<T>::get(&who) {
                None => {
                    let mut roles = Vec::new();
                    roles.push(role);
                    UserRoles::<T>::insert(&who, roles);
                },
                Some(mut roles) => {
                    let role_exists = roles.iter().any(|&r| r == role );
                    if !role_exists {
                        roles.push(role);
                        UserRoles::<T>::insert(&who, roles)
                    }
                }
            }
            
            Self::deposit_event(Event::RoleAssigned(who.clone(), role));
            Ok(().into())
        }

        // TODO: 
        // #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        // pub fn unassign_role(origin: OriginFor<T>, role: super::Role) -> DispatchResultWithPostInfo {
        //      
        // }
    }

    #[pallet::error]
    pub enum Error<T> {
        // This pallet does not return error from here
    }

    #[pallet::event]
    #[pallet::metadata()]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // This pallet does not emit events
        // [who, Role]
        RoleAssigned(T::AccountId, Role)
    }
    
}

#[macro_use]
extern crate lazy_static; // https://crates.io/crates/lazy_static
use sp_std::collections::btree_map::{BTreeMap, Entry};
use sp_std::marker::PhantomData;
use sp_std::fmt::Debug;
use sp_std::vec;
use sp_std::vec::Vec;
use frame_support::codec::{Encode, Decode};
use sp_runtime::traits::{
    SignedExtension, DispatchInfoOf, Dispatchable
};
use sp_runtime::transaction_validity::{
    ValidTransaction, TransactionValidityError,
    InvalidTransaction, TransactionValidity,
    TransactionPriority, TransactionLongevity,
};
use frame_support::weights::DispatchInfo;
use frame_support::traits::GetCallMetadata;
// For debugging --
use sp_runtime::print;
use frame_support::debug;


impl<T: Config> Pallet<T> {
    pub fn validate_access_to_call(who: &T::AccountId, pallet_name: &'static str, function_name: &'static str) -> bool {
        // Check if pallet function is in PUBLIC_ACCESS
        if PUBLIC_ACCESS.iter().any(|&x| x.0 == pallet_name && x.1 == function_name) {
            return true;
        }

        // Get user's roles
        let user_roles = UserRoles::<T>::get(who);
        if user_roles == None {
            return false;
        }
        debug::info!("-- validate_access_to_call --> user_roles -> {:?}", user_roles);
        // FIXME: Make role check more efficient.
        // maybe change the RBAC structure to:
        // { key: "Palletname:function_name", value: Role }
        // for each role check if it has access to pallet_name and function_name
        user_roles.unwrap().iter().any(|&role| {
            let role_access_list = RBAC.get(&role);
            /*
            debug::info!("-- iterating user_roles");
            debug::info!("   -- user has role : {:?} ", role);
            debug::info!("   -- role has access_list : {:?}", role_access_list);
            */
            match role_access_list {
                None => false,
                Some(access_list) => {
                    debug::info!("-- iterating access_list -- ");
                    let has_access = access_list.iter().any(|&access| {
                        /*
                        debug::info!("-- access is {:?} ", access);
                        debug::info!("-- pallet_name is {:?}", pallet_name);
                        debug::info!("-- function_name is {:?}", function_name);
                        */
                        access.0 == pallet_name && access.1 == function_name
                    });
                    return has_access;
                }
            }
        })
    }
}


#[derive(Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Encode, Decode, Debug)]
pub enum Role {
    Customer,
    Lab,
    Hospital,
    Doctor,
}

// Pallet name should be spelled as in runtime's construct_runtime! macro declaration
lazy_static! {
    // List of roles and pallet functions it has access to
    #[derive(Debug)]
    pub static ref RBAC: BTreeMap<Role, Vec<(&'static str, &'static str)>> = {
        let role_access_list: Vec<(Role, (&str, &str))> = vec![
            // Role         , pallet name,  function_name
            // Customers -- 
            (Role::Customer,    ("Orders",      "create_order")),
            (Role::Customer,    ("Orders",      "pay_order")),
            (Role::Customer,    ("Orders",      "refund_order")),

            
            // Labs --
            (Role::Lab,         ("Labs",        "register_lab")),
            (Role::Lab,         ("Labs",        "update_lab")),
            (Role::Lab,         ("Services",    "create_service")),
            (Role::Lab,         ("Services",    "update_service")),
            (Role::Lab,         ("Services",    "delete_service")),
            (Role::Lab,         ("Orders",      "fulfill_order")),
            (Role::Lab,         ("Specimen",    "receive")),
            (Role::Lab,         ("Specimen",    "reject")),
            (Role::Lab,         ("Specimen",    "process")),


            // Hospitals -- 
            //(Role::Hospital,    ("TODO",                "TODO")),

            // Doctors --
            //(Role::Doctor,    ("TODO",                "TODO")),
        ];
        
        let mut rbac = BTreeMap::new();
        // rbac.insert(Role::Lab, vec![("TemplateModule",       "do_something")]);
        for role_access in role_access_list.iter() {
            match rbac.entry(role_access.0) {
                Entry::Vacant(role) => {
                    debug::info!("Entry::Vacant(role) -> role = {:?}", role);
                    role.insert(vec![role_access.1]);
                },
                Entry::Occupied(mut role) => { 
                    debug::info!("Entry::Occupied(mut role) -> role = : {:?}", role);
                    role.get_mut().push(role_access.1);
                }
            }
        }

        //let _initialize = rbac.get(&Role::Customer);
        //debug::info!("rbac: {:?}", rbac);
        rbac
    };

    // List of public access pallet functions
    #[derive(Debug)]
    pub static ref PUBLIC_ACCESS: Vec<(&'static str, &'static str)> = {
        let public_access_list: Vec<(&str, &str)> = vec![
            ("RBAC", "assign_role")
        ];

        public_access_list
    };
}

#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct Authorize<T: Config + Send + Sync>(PhantomData<T>);

impl<T: Config + Send + Sync> Debug for Authorize<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
            write!(f, "Authorize")
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
            Ok(())
    }
}

impl<T: Config + Send + Sync> SignedExtension for Authorize<T> where
T::Call: Dispatchable<Info=DispatchInfo> + GetCallMetadata {
    type AccountId = T::AccountId;
    type Call = T::Call;
    type AdditionalSigned = ();
    type Pre = ();
    const IDENTIFIER: &'static str = "Authorize";


    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> { Ok(()) }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> TransactionValidity {
        let metadata = call.get_call_metadata();

        print("---- ---- RBAC  --- -----");
        debug::info!("{:?}", *RBAC);
        debug::info!("-- In rbac pallet --> Metadata --- {:?}", metadata);

        let pallet_name = metadata.pallet_name;
        let function_name = metadata.function_name;

        // Check if who has valid role for pallet_name and function_name
        // - check who's roles
        // - check if role has access to (pallet_name, function_name)
        let has_access = Pallet::<T>::validate_access_to_call(who, pallet_name, function_name);

        if has_access {
            Ok(ValidTransaction {
                priority: info.weight as TransactionPriority,
                longevity: TransactionLongevity::max_value(),
                propagate: true,
                ..Default::default()
            })
        } else {
            print("Access Denied!");
            debug::info!("------ Access Denied -------");
            Err(InvalidTransaction::Call.into())
        }
    }
}

