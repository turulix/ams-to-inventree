# MachineProperty

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Key of the property | 
**value** | **String** | Value of the property | 
**group** | Option<**String**> | Grouping of the property | [optional]
**r#type** | Option<[**models::MachinePropertyTypeEnum**](MachinePropertyTypeEnum.md)> | Type of the property  * `str` - str * `bool` - bool * `progress` - progress * `int` - int * `float` - float | [optional][default to Str]
**max_progress** | Option<**i32**> | Maximum value for progress type, required if type=progress | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


