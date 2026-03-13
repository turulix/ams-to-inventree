# SalesOrderLineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**link** | Option<**String**> | Link to external page | [optional]
**notes** | Option<**String**> | Line item notes | [optional]
**order** | **i32** | Sales Order | 
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**quantity** | **f64** |  | 
**reference** | Option<**String**> | Line item reference | [optional]
**target_date** | Option<[**String**](String.md)> |  | [optional]
**order_detail** | Option<[**models::SalesOrder**](SalesOrder.md)> |  | [optional][readonly]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**allocated** | **f64** |  | [readonly]
**customer_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**overdue** | Option<**bool**> |  | [optional][readonly]
**part** | Option<**i32**> | Part | [optional]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**sale_price** | Option<**String**> |  | [optional]
**sale_price_currency** | Option<**String**> | Sale price currency  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**shipped** | **f64** |  | [readonly]
**available_stock** | **f64** |  | [readonly]
**available_variant_stock** | **f64** |  | [readonly]
**building** | **f64** |  | [readonly]
**on_order** | **f64** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


