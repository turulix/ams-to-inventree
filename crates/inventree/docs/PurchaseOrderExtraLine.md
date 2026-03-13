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
**price_currency** | Option<**String**> | Select currency from available options | [optional]
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**quantity** | **f64** |  | 
**reference** | Option<**String**> | Line item reference | [optional]
**target_date** | Option<[**String**](String.md)> | Target date for this line item (leave blank to use the target date from the order) | [optional]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


