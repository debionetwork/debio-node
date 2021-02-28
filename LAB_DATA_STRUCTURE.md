Lab
id: AccountId
name: String
data: String (link to ipfs)

LabServices
labId: AccountId
services: Vec<Service.id>

Service
id: Some Hash
labId: Lab.AccountId
price: Units
data: String (link to ipfs)

Order
id: Some Hash
specimenNumber: String
customerId: AccountId
serviceId: Service.id
status: String

CustomerOrders
customerId: AccountId
orders: Vec<Order.id>

Escrow
id: AccountId
balance: Units


