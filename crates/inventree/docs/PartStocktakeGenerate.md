# PartStocktakeGenerate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**part** | Option<**i32**> | Select a part to generate stocktake information for that part (and any variant parts) | [optional]
**category** | Option<**i32**> | Select a category to include all parts within that category (and subcategories) | [optional]
**location** | Option<**i32**> | Select a location to include all parts with stock in that location (including sub-locations) | [optional]
**generate_entry** | Option<**bool**> | Save stocktake entries for the selected parts | [optional][default to false]
**generate_report** | Option<**bool**> | Generate a stocktake report for the selected parts | [optional][default to false]
**output** | [**models::DataOutput**](DataOutput.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


