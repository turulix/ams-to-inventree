# PatchedSelectionList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**name** | Option<**String**> | Name of the selection list | [optional]
**description** | Option<**String**> | Description of the selection list | [optional]
**active** | Option<**bool**> | Can this selection list be used? | [optional]
**locked** | Option<**bool**> | Is this selection list locked? | [optional]
**source_plugin** | Option<**i32**> | Plugin which provides the selection list | [optional]
**source_string** | Option<**String**> | Optional string identifying the source used for this list | [optional]
**default** | Option<[**models::SelectionEntry**](SelectionEntry.md)> |  | [optional][readonly]
**created** | Option<**String**> | Date and time that the selection list was created | [optional][readonly]
**last_updated** | Option<**String**> | Date and time that the selection list was last updated | [optional][readonly]
**choices** | Option<[**Vec<models::SelectionEntry>**](SelectionEntry.md)> |  | [optional]
**entry_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


