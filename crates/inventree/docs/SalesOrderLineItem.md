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
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**allocated** | **f64** |  | [readonly]
**overdue** | Option<**bool**> |  | [optional][readonly]
**part** | Option<**i32**> | Part | [optional]
**sale_price** | Option<**String**> |  | [optional]
**sale_price_currency** | Option<**String**> | Sale price currency | [optional]
**shipped** | **f64** |  | [readonly]
**available_stock** | **f64** |  | [readonly]
**available_variant_stock** | **f64** |  | [readonly]
**building** | **f64** |  | [readonly]
**on_order** | **f64** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


