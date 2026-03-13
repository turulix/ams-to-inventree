# PatchedSalesOrderAllocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**item** | Option<**i32**> | Select stock item to allocate | [optional]
**quantity** | Option<**f64**> |  | [optional]
**shipment** | Option<**i32**> | Sales order shipment reference | [optional]
**line** | Option<**i32**> |  | [optional][readonly]
**part** | Option<**i32**> |  | [optional][readonly]
**order** | Option<**i32**> |  | [optional][readonly]
**serial** | Option<**String**> |  | [optional][readonly]
**location** | Option<**i32**> |  | [optional][readonly]
**item_detail** | Option<[**models::StockItem**](StockItem.md)> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**order_detail** | Option<[**models::SalesOrder**](SalesOrder.md)> |  | [optional][readonly]
**customer_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**location_detail** | Option<[**models::LocationBrief**](LocationBrief.md)> |  | [optional][readonly]
**shipment_detail** | Option<[**models::SalesOrderShipment**](SalesOrderShipment.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


