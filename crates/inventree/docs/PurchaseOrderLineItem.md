# PurchaseOrderLineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**link** | Option<**String**> | Link to external page | [optional]
**notes** | Option<**String**> | Line item notes | [optional]
**order** | **i32** | Purchase Order | 
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**quantity** | **f64** |  | 
**reference** | Option<**String**> | Line item reference | [optional]
**target_date** | Option<[**String**](String.md)> |  | [optional]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**part** | Option<**i32**> |  | 
**build_order** | Option<**i32**> | External Build Order to be fulfilled by this line item | [optional]
**overdue** | Option<**bool**> |  | [optional][readonly]
**received** | **f64** |  | [readonly][default to 0.0]
**purchase_price** | Option<**String**> |  | [optional]
**purchase_price_currency** | Option<**String**> | Purchase price currency | [optional]
**auto_pricing** | Option<**bool**> | Automatically calculate purchase price based on supplier part data | [optional][default to true]
**destination** | Option<**i32**> | Destination for received items | [optional]
**total_price** | **f64** |  | [readonly]
**merge_items** | Option<**bool**> | Merge items with the same part, destination and target date into one line item | [optional][default to true]
**sku** | Option<**String**> |  | [optional][readonly]
**mpn** | Option<**String**> |  | [optional][readonly]
**ipn** | Option<**String**> |  | [optional][readonly]
**internal_part** | **i32** |  | [readonly]
**internal_part_name** | **String** |  | [readonly]
**build_order_detail** | Option<[**models::Build**](Build.md)> |  | [optional][readonly]
**destination_detail** | Option<[**models::LocationBrief**](LocationBrief.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


