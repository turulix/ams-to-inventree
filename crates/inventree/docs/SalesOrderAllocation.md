# SalesOrderAllocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**item** | **i32** | Select stock item to allocate | 
**quantity** | **f64** |  | 
**shipment** | Option<**i32**> | Sales order shipment reference | [optional]
**line** | **i32** |  | [readonly]
**part** | **i32** |  | [readonly]
**order** | **i32** |  | [readonly]
**serial** | Option<**String**> |  | [optional][readonly]
**location** | **i32** |  | [readonly]
**shipment_detail** | Option<[**models::SalesOrderShipment**](SalesOrderShipment.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


