# PatchedSalesOrderLineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**link** | Option<**String**> | Link to external page | [optional]
**notes** | Option<**String**> | Line item notes | [optional]
**order** | Option<**i32**> | Sales Order | [optional]
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**quantity** | Option<**f64**> |  | [optional]
**reference** | Option<**String**> | Line item reference | [optional]
**target_date** | Option<[**String**](String.md)> |  | [optional]
**order_detail** | Option<[**models::SalesOrder**](SalesOrder.md)> |  | [optional][readonly]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**allocated** | Option<**f64**> |  | [optional][readonly]
**customer_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**overdue** | Option<**bool**> |  | [optional][readonly]
**part** | Option<**i32**> | Part | [optional]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**sale_price** | Option<**String**> |  | [optional]
**sale_price_currency** | Option<**String**> | Sale price currency  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**shipped** | Option<**f64**> |  | [optional][readonly]
**available_stock** | Option<**f64**> |  | [optional][readonly]
**available_variant_stock** | Option<**f64**> |  | [optional][readonly]
**building** | Option<**f64**> |  | [optional][readonly]
**on_order** | Option<**f64**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


