# Enrichment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | Describes what the enrichment step does. | [optional] [default to ]
**destination_field** | **String** | Field where enrichments will be stored. This field must already exist or be at most 1 level deeper than an existing field. For example, if `text` is a top-level field with no sub-fields, `text.foo` is a valid destination but `text.foo.bar` is not. | 
**source_field** | **String** | Field to be enriched. | 
**overwrite** | **bool** | Indicates that the enrichments will overwrite the destination_field field if it already exists. | [optional] [default to false]
**enrichment** | **String** | Name of the enrichment service to call. Current options are `natural_language_understanding` and `elements`.   When using `natual_language_understanding`, the **options** object must contain Natural Language Understanding options.   When using `elements` the **options** object must contain Element Classification options. Additionally, when using the `elements` enrichment the configuration specified and files ingested must meet all the criteria specified in [the documentation](https://console.bluemix.net/docs/services/discovery/element-classification.html)     Previous API versions also supported `alchemy_language`. | 
**ignore_downstream_errors** | **bool** | If true, then most errors generated during the enrichment process will be treated as warnings and will not cause the document to fail processing. | [optional] [default to false]
**options** | [***::models::EnrichmentOptions**](EnrichmentOptions.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


