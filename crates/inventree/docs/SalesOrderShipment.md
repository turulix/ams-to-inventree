# SalesOrderShipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**order** | **i32** | Sales Order | 
**allocated_items** | Option<**i32**> |  | [optional][readonly]
**shipment_date** | Option<[**String**](String.md)> | Date of shipment | [optional]
**shipment_address** | Option<**i32**> | Shipping address for this shipment | [optional]
**delivery_date** | Option<[**String**](String.md)> | Date of delivery of shipment | [optional]
**checked_by** | Option<**i32**> | User who checked this shipment | [optional]
**reference** | Option<**String**> | Shipment number | [optional][default to 1]
**tracking_number** | Option<**String**> | Shipment tracking information | [optional]
**invoice_number** | Option<**String**> | Reference number for associated invoice | [optional]
**barcode_hash** | Option<**String**> | Unique hash of barcode data | [optional]
**link** | Option<**String**> | Link to external page | [optional]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]
**checked_by_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**shipment_address_detail** | Option<[**models::AddressBrief**](AddressBrief.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


