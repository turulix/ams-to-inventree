# StockItemTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**stock_item** | **i32** |  | 
**result** | Option<**bool**> | Test result | [optional]
**value** | Option<**String**> | Test output value | [optional]
**attachment** | Option<**String**> | Test result attachment | [optional]
**notes** | Option<**String**> | Test notes | [optional]
**test_station** | Option<**String**> | The identifier of the test station where the test was performed | [optional]
**started_datetime** | Option<**String**> | The timestamp of the test start | [optional]
**finished_datetime** | Option<**String**> | The timestamp of the test finish | [optional]
**user** | Option<**i32**> |  | [optional][readonly]
**user_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**date** | **String** |  | [readonly]
**template** | Option<**i32**> | Template | [optional]
**template_detail** | Option<[**models::PartTestTemplate**](PartTestTemplate.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


