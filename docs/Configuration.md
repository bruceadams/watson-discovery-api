# Configuration

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration_id** | **String** | The unique identifier of the configuration | [optional] 
**name** | **String** | The name of the configuration. | [default to ]
**created** | **String** | The creation date of the configuration in the format yyyy-MM-dd'T'HH:mm:ss.SSS'Z' | [optional] 
**updated** | **String** | The timestamp of when the configuration was last updated in the format yyyy-MM-dd'T'HH:mm:ss.SSS'Z' | [optional] 
**description** | **String** | The description of the configuration, if available. | [optional] 
**conversions** | [***::models::Conversions**](Conversions.md) |  | [optional] 
**enrichments** | [**Vec<::models::Enrichment>**](Enrichment.md) | An array of document enrichment settings for the configuration. | [optional] 
**normalizations** | [**Vec<::models::NormalizationOperation>**](NormalizationOperation.md) | Defines operations that can be used to transform the final output JSON into a normalized form. Operations are executed in the order that they appear in the array. | [optional] 
**source** | [***::models::Source**](Source.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


