# PatchedPartBomValidate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**bom_validated** | Option<**bool**> | Is the BOM for this part valid? | [optional][readonly]
**bom_checksum** | Option<**String**> | Stored BOM checksum | [optional][readonly]
**bom_checked_by** | Option<**i32**> |  | [optional][readonly]
**bom_checked_by_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**bom_checked_date** | Option<[**String**](String.md)> |  | [optional][readonly]
**valid** | Option<**bool**> | Validate entire Bill of Materials | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


