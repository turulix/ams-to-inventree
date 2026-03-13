# PurchaseOrderExtraLine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**description** | Option<**String**> | Line item description (optional) | [optional]
**link** | Option<**String**> | Link to external page | [optional]
**notes** | Option<**String**> | Line item notes | [optional]
**order** | **i32** | Purchase Order | 
**price** | Option<**String**> |  | [optional]
**price_currency** | Option<**String**> | Select currency from available options  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**quantity** | **f64** |  | 
**reference** | Option<**String**> | Line item reference | [optional]
**target_date** | Option<[**String**](String.md)> | Target date for this line item (leave blank to use the target date from the order) | [optional]
**order_detail** | Option<[**models::PurchaseOrder**](PurchaseOrder.md)> |  | [optional][readonly]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


