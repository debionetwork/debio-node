# Appchain Function Spec

### As general user (customer / lab) 
- Wallet Binding : Pallet User Profile 
    - set_eth_address(eth_address) : bind between polkadot and eth address 

### As lab
- Create, Update, Delete lab : Pallet Lab
    - registerLab(labInfo) 
    - updateLab(labInfo) 
    - deregisterLab() 
- Create, Update, Delete service : Pallet Service 
    - createService(service_info) 
    - updateService(service_id, service_info) 
    - deleteService(service_id) 
- Create, Update, Delete lab certification : Pallet certification 
    - createCertification(certification_info) 
    - updateCertification(certification_id, certification_info) deleteCertification(certification_id) 
- Process order : Pallet Genetic Testing 
    - rejectDnaSample(tracking_id, rejected_title, rejected_description) 
    - processDnaSample(tracking_id, status) 
    - submitTestResult(tracking_id, submission) 
    - submitIndependentTestResult(submission)
    - submitDataStakingDetails(data_hash) 
- Finished order : Pallet Order
    - fulfillOrder(order_id) 

### As customer 
- Create, Cancel order : Pallet Order 
    - createOrder(service_id,price_index, customer_box_public_key) 
    - cancelOrder(order_id) 
- Create, Update, Delete EMR : Pallet EMR 
    - addElectronicMedicalRecord() 
    - removeElectronicMedicalRecord() 
    - addElectronicMedicalRecordFile(title, description) 
    - removeElectronicMedicalRecord(electronic_medical_record_file_id) 

### As Escrow
- Set Order to Paid : Pallet Order
    - setOrderPaid(orderId)
- Set Order to Refunded : Pallet Order
    - setOrderRefunded(orderId)