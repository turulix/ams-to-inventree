# PartBrief

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**ipn** | Option<**String**> | Internal Part Number | [optional]
**barcode_hash** | **String** | Unique hash of barcode data | [readonly]
**category_default_location** | Option<**i32**> |  | [optional][readonly]
**default_location** | Option<**i32**> | Where is this item normally stored? | [optional]
**default_expiry** | Option<**u64**> | Expiry time (in days) for stock items of this part | [optional]
**name** | **String** | Part name | 
**revision** | Option<**String**> |  | [optional][default to ]
**full_name** | **String** | Format a 'full name' for this Part based on the format PART_NAME_FORMAT defined in InvenTree settings. | [readonly]
**description** | Option<**String**> | Part description (optional) | [optional]
**image** | Option<**String**> |  | [optional][readonly]
**thumbnail** | **String** |  | [readonly]
**active** | Option<**bool**> | Is this part active? | [optional]
**locked** | Option<**bool**> | Locked parts cannot be edited | [optional]
**assembly** | Option<**bool**> | Can this part be built from other parts? | [optional]
**component** | Option<**bool**> | Can this part be used to build other parts? | [optional]
**minimum_stock** | Option<**String**> | Minimum allowed stock level | [optional]
**is_template** | Option<**bool**> | Is this part a template part? | [optional]
**purchaseable** | Option<**bool**> | Can this part be purchased from external suppliers? | [optional]
**salable** | Option<**bool**> | Can this part be sold to customers? | [optional]
**testable** | Option<**bool**> | Can this part have test results recorded against it? | [optional]
**trackable** | Option<**bool**> | Does this part have tracking for unique items? | [optional]
**r#virtual** | Option<**bool**> | Is this a virtual part, such as a software product or license? | [optional]
**units** | Option<**String**> | Units of measure for this part | [optional]
**pricing_min** | Option<**String**> |  | [optional][readonly]
**pricing_max** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


