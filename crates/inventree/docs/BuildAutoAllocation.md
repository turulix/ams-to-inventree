# BuildAutoAllocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location** | Option<**i32**> | Stock location where parts are to be sourced (leave blank to take from any location) | [optional]
**exclude_location** | Option<**i32**> | Exclude stock items from this selected location | [optional]
**interchangeable** | Option<**bool**> | Stock items in multiple locations can be used interchangeably | [optional][default to false]
**substitutes** | Option<**bool**> | Allow allocation of substitute parts | [optional][default to true]
**optional_items** | Option<**bool**> | Allocate optional BOM items to build order | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


