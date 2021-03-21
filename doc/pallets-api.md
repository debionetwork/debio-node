# Labs

## Dispatchable Calls
### register_lab
Params:
- lab_name: String
- country: String
- city: String
- address: String
- latitude: String?
- longitude: String?
- profile_image: String?
- is_verified: Boolean? TODO: Remove this. Make lab verification in a separate call

### update_lab
Params:
- lab_name: String
- country: String
- city: String
- address: String
- latitude: String?
- longitude: String?
- profile_image: String?

## Storage
### Labs
Key:
- account_id: AccountId

---
# Services
## Dispatchable Calls
### create_service
Params:
- name: String
- price: Uint128
- description: String
- long_description: String?
- image: String?

### update_service
- service_id: H256
- name: String
- price: Uint128
- description: String
- long_description: String?
- image: String?

### delete_service
- service_id: H256

## Storage
### Services
key: 
- service_id: H256

---

# Orders
## Dispatchable Calls
### create_order
Params:
- service_id: H256

### pay_order
Params:
- order_id: H256

### fulfill_order
Params:
- order_id: H256

### refund_order
Params:
- order_id: H256

## Storage
### Orders
Keys:
- order_id: H256

### CustomerOrders
Keys:
- customer_id: AccountId

### LabOrders
- lab_id: AccountId

---

# Escrow
Escrow accounts are automatically created when orders are created.  
When order is paid, the funds are transferred to an escrow account.  
When order is fulfilled, the funds in an escrow account are released to the seller's account.  
When order is refunded, the funds in an escrow account are given back to the customer's account.  

## Dispatchable Calls
None

## Storage
### Escrows
Keys:
- order_id: H256

---

# Specimen
When order is created, a specimen is created with a status of "Sending".  
The specimen is identified by the order_id.  
TODO: specimen number should be human readable alphanumeric id that maps to the order and the customer (specimen owner)

## Dispatchable Calls
### receive_specimen
Params:
- order_id: H256

### reject_specimen
Params:
- order_id: H256
- rejection_reason: String

### process_specimen
Params:
- order_id: H256
- result_file: String
- result_report: String

## Storage
### Specimens
Keys:
- order_id: H256