# CustomState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**key** | **i32** | Numerical value that will be saved in the models database | 
**name** | **String** | Name of the state | 
**label** | **String** | Label that will be displayed in the frontend | 
**color** | Option<[**models::ColorEnum**](ColorEnum.md)> | Color that will be displayed in the frontend  * `primary` - primary * `secondary` - secondary * `success` - success * `danger` - danger * `warning` - warning * `info` - info * `dark` - dark | [optional]
**logical_key** | **i32** | State logical key that is equal to this custom state in business logic | 
**model** | Option<**i32**> | Model this state is associated with | [optional]
**model_name** | **String** |  | [readonly]
**reference_status** | [**models::ReferenceStatusEnum**](ReferenceStatusEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


