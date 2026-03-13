# BarcodeScanResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**data** | **String** | Barcode data | [readonly]
**timestamp** | **String** | Date and time of the barcode scan | [readonly]
**endpoint** | Option<**String**> | URL endpoint which processed the barcode | [optional][readonly]
**context** | Option<**serde_json::Value**> | Context data for the barcode scan | [optional][readonly]
**response** | Option<**serde_json::Value**> | Response data from the barcode scan | [optional][readonly]
**result** | **bool** | Was the barcode scan successful? | [readonly]
**user** | Option<**i32**> | User who scanned the barcode | [optional][readonly]
**user_detail** | [**models::User**](User.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


