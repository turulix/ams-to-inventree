# PartBomValidate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**bom_validated** | **bool** | Is the BOM for this part valid? | [readonly]
**bom_checksum** | **String** | Stored BOM checksum | [readonly]
**bom_checked_by** | Option<**i32**> |  | [optional][readonly]
**bom_checked_by_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**bom_checked_date** | Option<[**String**](String.md)> |  | [optional][readonly]
**valid** | Option<**bool**> | Validate entire Bill of Materials | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


