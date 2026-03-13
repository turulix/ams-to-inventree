# PatchedParameterTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**name** | Option<**String**> | Parameter Name | [optional]
**units** | Option<**String**> | Physical units for this parameter | [optional]
**description** | Option<**String**> | Parameter description | [optional]
**model_type** | Option<**ModelType**> |  (enum: build.build, company.company, company.supplierpart, company.manufacturerpart, order.purchaseorder, order.salesorder, order.returnorder, part.part, stock.stocklocation, , ) | [optional][default to Empty]
**checkbox** | Option<**bool**> | Is this parameter a checkbox? | [optional]
**choices** | Option<**String**> | Valid choices for this parameter (comma-separated) | [optional]
**selectionlist** | Option<**i32**> | Selection list for this parameter | [optional]
**enabled** | Option<**bool**> | Is this parameter template enabled? | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


