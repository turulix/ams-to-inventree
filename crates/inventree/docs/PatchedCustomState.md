# PatchedCustomState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**key** | Option<**i64**> | Numerical value that will be saved in the models database | [optional]
**name** | Option<**String**> | Name of the state | [optional]
**label** | Option<**String**> | Label that will be displayed in the frontend | [optional]
**color** | Option<[**models::ColorEnum**](ColorEnum.md)> | Color that will be displayed in the frontend  * `primary` - primary * `secondary` - secondary * `success` - success * `danger` - danger * `warning` - warning * `info` - info * `dark` - dark | [optional]
**logical_key** | Option<**i64**> | State logical key that is equal to this custom state in business logic | [optional]
**model** | Option<**i32**> | Model this state is associated with | [optional]
**model_name** | Option<**String**> |  | [optional][readonly]
**reference_status** | Option<[**models::ReferenceStatusEnum**](ReferenceStatusEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


