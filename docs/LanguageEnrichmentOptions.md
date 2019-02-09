# LanguageEnrichmentOptions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extract** | **Vec<String>** | A comma-separated list of analyses that will be applied when using the `alchemy_language` enrichment. See the service documentation for details on each extract option.  Possible values include:    * entity   * keyword   * taxonomy   * concept   * relation   * doc-sentiment   * doc-emotion   * typed-rels | [optional] 
**sentiment** | **bool** |  | [optional] [default to false]
**quotations** | **bool** |  | [optional] [default to false]
**show_source_text** | **bool** |  | [optional] [default to false]
**hierarchical_typed_relations** | **bool** |  | [optional] [default to false]
**model** | **String** | Required when using the `typed-rel` extract option. Should be set to the ID of a previously published custom Watson Knowledge Studio model. | [optional] 
**language** | **String** | If provided, then do not attempt to detect the language of the input document. Instead, assume the language is the one specified in this field.  You can set this property to work around `unsupported-text-language` errors.  Supported languages include English, German, French, Italian, Portuguese, Russian, Spanish and Swedish. Supported language codes are the ISO-639-1, ISO-639-2, ISO-639-3, and the plain english name of the language (for example \"russian\"). | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


