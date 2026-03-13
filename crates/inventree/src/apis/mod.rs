use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod action_api;
pub mod admin_api;
pub mod attachment_api;
pub mod background_task_api;
pub mod barcode_api;
pub mod bom_api;
pub mod build_api;
pub mod company_api;
pub mod contenttype_api;
pub mod currency_api;
pub mod data_output_api;
pub mod default_api;
pub mod email_api;
pub mod error_report_api;
pub mod flags_api;
pub mod generate_api;
pub mod generic_api;
pub mod icons_api;
pub mod importer_api;
pub mod label_api;
pub mod license_api;
pub mod locate_api;
pub mod machine_api;
pub mod metadata_api;
pub mod news_api;
pub mod notes_image_upload_api;
pub mod notifications_api;
pub mod order_api;
pub mod parameter_api;
pub mod part_api;
pub mod plugin_api;
pub mod plugins_api;
pub mod project_code_api;
pub mod report_api;
pub mod search_api;
pub mod selection_api;
pub mod settings_api;
pub mod stock_api;
pub mod supplier_api;
pub mod system_api;
pub mod system_internal_api;
pub mod units_api;
pub mod user_api;
pub mod version_api;
pub mod version_text_api;
pub mod webhook_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn action_api(&self) -> &dyn action_api::ActionApi;
    fn admin_api(&self) -> &dyn admin_api::AdminApi;
    fn attachment_api(&self) -> &dyn attachment_api::AttachmentApi;
    fn background_task_api(&self) -> &dyn background_task_api::BackgroundTaskApi;
    fn barcode_api(&self) -> &dyn barcode_api::BarcodeApi;
    fn bom_api(&self) -> &dyn bom_api::BomApi;
    fn build_api(&self) -> &dyn build_api::BuildApi;
    fn company_api(&self) -> &dyn company_api::CompanyApi;
    fn contenttype_api(&self) -> &dyn contenttype_api::ContenttypeApi;
    fn currency_api(&self) -> &dyn currency_api::CurrencyApi;
    fn data_output_api(&self) -> &dyn data_output_api::DataOutputApi;
    fn default_api(&self) -> &dyn default_api::DefaultApi;
    fn email_api(&self) -> &dyn email_api::EmailApi;
    fn error_report_api(&self) -> &dyn error_report_api::ErrorReportApi;
    fn flags_api(&self) -> &dyn flags_api::FlagsApi;
    fn generate_api(&self) -> &dyn generate_api::GenerateApi;
    fn generic_api(&self) -> &dyn generic_api::GenericApi;
    fn icons_api(&self) -> &dyn icons_api::IconsApi;
    fn importer_api(&self) -> &dyn importer_api::ImporterApi;
    fn label_api(&self) -> &dyn label_api::LabelApi;
    fn license_api(&self) -> &dyn license_api::LicenseApi;
    fn locate_api(&self) -> &dyn locate_api::LocateApi;
    fn machine_api(&self) -> &dyn machine_api::MachineApi;
    fn metadata_api(&self) -> &dyn metadata_api::MetadataApi;
    fn news_api(&self) -> &dyn news_api::NewsApi;
    fn notes_image_upload_api(&self) -> &dyn notes_image_upload_api::NotesImageUploadApi;
    fn notifications_api(&self) -> &dyn notifications_api::NotificationsApi;
    fn order_api(&self) -> &dyn order_api::OrderApi;
    fn parameter_api(&self) -> &dyn parameter_api::ParameterApi;
    fn part_api(&self) -> &dyn part_api::PartApi;
    fn plugin_api(&self) -> &dyn plugin_api::PluginApi;
    fn plugins_api(&self) -> &dyn plugins_api::PluginsApi;
    fn project_code_api(&self) -> &dyn project_code_api::ProjectCodeApi;
    fn report_api(&self) -> &dyn report_api::ReportApi;
    fn search_api(&self) -> &dyn search_api::SearchApi;
    fn selection_api(&self) -> &dyn selection_api::SelectionApi;
    fn settings_api(&self) -> &dyn settings_api::SettingsApi;
    fn stock_api(&self) -> &dyn stock_api::StockApi;
    fn supplier_api(&self) -> &dyn supplier_api::SupplierApi;
    fn system_api(&self) -> &dyn system_api::SystemApi;
    fn system_internal_api(&self) -> &dyn system_internal_api::SystemInternalApi;
    fn units_api(&self) -> &dyn units_api::UnitsApi;
    fn user_api(&self) -> &dyn user_api::UserApi;
    fn version_api(&self) -> &dyn version_api::VersionApi;
    fn version_text_api(&self) -> &dyn version_text_api::VersionTextApi;
    fn webhook_api(&self) -> &dyn webhook_api::WebhookApi;
}

pub struct ApiClient {
    action_api: Box<dyn action_api::ActionApi>,
    admin_api: Box<dyn admin_api::AdminApi>,
    attachment_api: Box<dyn attachment_api::AttachmentApi>,
    background_task_api: Box<dyn background_task_api::BackgroundTaskApi>,
    barcode_api: Box<dyn barcode_api::BarcodeApi>,
    bom_api: Box<dyn bom_api::BomApi>,
    build_api: Box<dyn build_api::BuildApi>,
    company_api: Box<dyn company_api::CompanyApi>,
    contenttype_api: Box<dyn contenttype_api::ContenttypeApi>,
    currency_api: Box<dyn currency_api::CurrencyApi>,
    data_output_api: Box<dyn data_output_api::DataOutputApi>,
    default_api: Box<dyn default_api::DefaultApi>,
    email_api: Box<dyn email_api::EmailApi>,
    error_report_api: Box<dyn error_report_api::ErrorReportApi>,
    flags_api: Box<dyn flags_api::FlagsApi>,
    generate_api: Box<dyn generate_api::GenerateApi>,
    generic_api: Box<dyn generic_api::GenericApi>,
    icons_api: Box<dyn icons_api::IconsApi>,
    importer_api: Box<dyn importer_api::ImporterApi>,
    label_api: Box<dyn label_api::LabelApi>,
    license_api: Box<dyn license_api::LicenseApi>,
    locate_api: Box<dyn locate_api::LocateApi>,
    machine_api: Box<dyn machine_api::MachineApi>,
    metadata_api: Box<dyn metadata_api::MetadataApi>,
    news_api: Box<dyn news_api::NewsApi>,
    notes_image_upload_api: Box<dyn notes_image_upload_api::NotesImageUploadApi>,
    notifications_api: Box<dyn notifications_api::NotificationsApi>,
    order_api: Box<dyn order_api::OrderApi>,
    parameter_api: Box<dyn parameter_api::ParameterApi>,
    part_api: Box<dyn part_api::PartApi>,
    plugin_api: Box<dyn plugin_api::PluginApi>,
    plugins_api: Box<dyn plugins_api::PluginsApi>,
    project_code_api: Box<dyn project_code_api::ProjectCodeApi>,
    report_api: Box<dyn report_api::ReportApi>,
    search_api: Box<dyn search_api::SearchApi>,
    selection_api: Box<dyn selection_api::SelectionApi>,
    settings_api: Box<dyn settings_api::SettingsApi>,
    stock_api: Box<dyn stock_api::StockApi>,
    supplier_api: Box<dyn supplier_api::SupplierApi>,
    system_api: Box<dyn system_api::SystemApi>,
    system_internal_api: Box<dyn system_internal_api::SystemInternalApi>,
    units_api: Box<dyn units_api::UnitsApi>,
    user_api: Box<dyn user_api::UserApi>,
    version_api: Box<dyn version_api::VersionApi>,
    version_text_api: Box<dyn version_text_api::VersionTextApi>,
    webhook_api: Box<dyn webhook_api::WebhookApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            action_api: Box::new(action_api::ActionApiClient::new(configuration.clone())),
            admin_api: Box::new(admin_api::AdminApiClient::new(configuration.clone())),
            attachment_api: Box::new(attachment_api::AttachmentApiClient::new(
                configuration.clone(),
            )),
            background_task_api: Box::new(background_task_api::BackgroundTaskApiClient::new(
                configuration.clone(),
            )),
            barcode_api: Box::new(barcode_api::BarcodeApiClient::new(configuration.clone())),
            bom_api: Box::new(bom_api::BomApiClient::new(configuration.clone())),
            build_api: Box::new(build_api::BuildApiClient::new(configuration.clone())),
            company_api: Box::new(company_api::CompanyApiClient::new(configuration.clone())),
            contenttype_api: Box::new(contenttype_api::ContenttypeApiClient::new(
                configuration.clone(),
            )),
            currency_api: Box::new(currency_api::CurrencyApiClient::new(configuration.clone())),
            data_output_api: Box::new(data_output_api::DataOutputApiClient::new(
                configuration.clone(),
            )),
            default_api: Box::new(default_api::DefaultApiClient::new(configuration.clone())),
            email_api: Box::new(email_api::EmailApiClient::new(configuration.clone())),
            error_report_api: Box::new(error_report_api::ErrorReportApiClient::new(
                configuration.clone(),
            )),
            flags_api: Box::new(flags_api::FlagsApiClient::new(configuration.clone())),
            generate_api: Box::new(generate_api::GenerateApiClient::new(configuration.clone())),
            generic_api: Box::new(generic_api::GenericApiClient::new(configuration.clone())),
            icons_api: Box::new(icons_api::IconsApiClient::new(configuration.clone())),
            importer_api: Box::new(importer_api::ImporterApiClient::new(configuration.clone())),
            label_api: Box::new(label_api::LabelApiClient::new(configuration.clone())),
            license_api: Box::new(license_api::LicenseApiClient::new(configuration.clone())),
            locate_api: Box::new(locate_api::LocateApiClient::new(configuration.clone())),
            machine_api: Box::new(machine_api::MachineApiClient::new(configuration.clone())),
            metadata_api: Box::new(metadata_api::MetadataApiClient::new(configuration.clone())),
            news_api: Box::new(news_api::NewsApiClient::new(configuration.clone())),
            notes_image_upload_api: Box::new(
                notes_image_upload_api::NotesImageUploadApiClient::new(configuration.clone()),
            ),
            notifications_api: Box::new(notifications_api::NotificationsApiClient::new(
                configuration.clone(),
            )),
            order_api: Box::new(order_api::OrderApiClient::new(configuration.clone())),
            parameter_api: Box::new(parameter_api::ParameterApiClient::new(
                configuration.clone(),
            )),
            part_api: Box::new(part_api::PartApiClient::new(configuration.clone())),
            plugin_api: Box::new(plugin_api::PluginApiClient::new(configuration.clone())),
            plugins_api: Box::new(plugins_api::PluginsApiClient::new(configuration.clone())),
            project_code_api: Box::new(project_code_api::ProjectCodeApiClient::new(
                configuration.clone(),
            )),
            report_api: Box::new(report_api::ReportApiClient::new(configuration.clone())),
            search_api: Box::new(search_api::SearchApiClient::new(configuration.clone())),
            selection_api: Box::new(selection_api::SelectionApiClient::new(
                configuration.clone(),
            )),
            settings_api: Box::new(settings_api::SettingsApiClient::new(configuration.clone())),
            stock_api: Box::new(stock_api::StockApiClient::new(configuration.clone())),
            supplier_api: Box::new(supplier_api::SupplierApiClient::new(configuration.clone())),
            system_api: Box::new(system_api::SystemApiClient::new(configuration.clone())),
            system_internal_api: Box::new(system_internal_api::SystemInternalApiClient::new(
                configuration.clone(),
            )),
            units_api: Box::new(units_api::UnitsApiClient::new(configuration.clone())),
            user_api: Box::new(user_api::UserApiClient::new(configuration.clone())),
            version_api: Box::new(version_api::VersionApiClient::new(configuration.clone())),
            version_text_api: Box::new(version_text_api::VersionTextApiClient::new(
                configuration.clone(),
            )),
            webhook_api: Box::new(webhook_api::WebhookApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn action_api(&self) -> &dyn action_api::ActionApi {
        self.action_api.as_ref()
    }
    fn admin_api(&self) -> &dyn admin_api::AdminApi {
        self.admin_api.as_ref()
    }
    fn attachment_api(&self) -> &dyn attachment_api::AttachmentApi {
        self.attachment_api.as_ref()
    }
    fn background_task_api(&self) -> &dyn background_task_api::BackgroundTaskApi {
        self.background_task_api.as_ref()
    }
    fn barcode_api(&self) -> &dyn barcode_api::BarcodeApi {
        self.barcode_api.as_ref()
    }
    fn bom_api(&self) -> &dyn bom_api::BomApi {
        self.bom_api.as_ref()
    }
    fn build_api(&self) -> &dyn build_api::BuildApi {
        self.build_api.as_ref()
    }
    fn company_api(&self) -> &dyn company_api::CompanyApi {
        self.company_api.as_ref()
    }
    fn contenttype_api(&self) -> &dyn contenttype_api::ContenttypeApi {
        self.contenttype_api.as_ref()
    }
    fn currency_api(&self) -> &dyn currency_api::CurrencyApi {
        self.currency_api.as_ref()
    }
    fn data_output_api(&self) -> &dyn data_output_api::DataOutputApi {
        self.data_output_api.as_ref()
    }
    fn default_api(&self) -> &dyn default_api::DefaultApi {
        self.default_api.as_ref()
    }
    fn email_api(&self) -> &dyn email_api::EmailApi {
        self.email_api.as_ref()
    }
    fn error_report_api(&self) -> &dyn error_report_api::ErrorReportApi {
        self.error_report_api.as_ref()
    }
    fn flags_api(&self) -> &dyn flags_api::FlagsApi {
        self.flags_api.as_ref()
    }
    fn generate_api(&self) -> &dyn generate_api::GenerateApi {
        self.generate_api.as_ref()
    }
    fn generic_api(&self) -> &dyn generic_api::GenericApi {
        self.generic_api.as_ref()
    }
    fn icons_api(&self) -> &dyn icons_api::IconsApi {
        self.icons_api.as_ref()
    }
    fn importer_api(&self) -> &dyn importer_api::ImporterApi {
        self.importer_api.as_ref()
    }
    fn label_api(&self) -> &dyn label_api::LabelApi {
        self.label_api.as_ref()
    }
    fn license_api(&self) -> &dyn license_api::LicenseApi {
        self.license_api.as_ref()
    }
    fn locate_api(&self) -> &dyn locate_api::LocateApi {
        self.locate_api.as_ref()
    }
    fn machine_api(&self) -> &dyn machine_api::MachineApi {
        self.machine_api.as_ref()
    }
    fn metadata_api(&self) -> &dyn metadata_api::MetadataApi {
        self.metadata_api.as_ref()
    }
    fn news_api(&self) -> &dyn news_api::NewsApi {
        self.news_api.as_ref()
    }
    fn notes_image_upload_api(&self) -> &dyn notes_image_upload_api::NotesImageUploadApi {
        self.notes_image_upload_api.as_ref()
    }
    fn notifications_api(&self) -> &dyn notifications_api::NotificationsApi {
        self.notifications_api.as_ref()
    }
    fn order_api(&self) -> &dyn order_api::OrderApi {
        self.order_api.as_ref()
    }
    fn parameter_api(&self) -> &dyn parameter_api::ParameterApi {
        self.parameter_api.as_ref()
    }
    fn part_api(&self) -> &dyn part_api::PartApi {
        self.part_api.as_ref()
    }
    fn plugin_api(&self) -> &dyn plugin_api::PluginApi {
        self.plugin_api.as_ref()
    }
    fn plugins_api(&self) -> &dyn plugins_api::PluginsApi {
        self.plugins_api.as_ref()
    }
    fn project_code_api(&self) -> &dyn project_code_api::ProjectCodeApi {
        self.project_code_api.as_ref()
    }
    fn report_api(&self) -> &dyn report_api::ReportApi {
        self.report_api.as_ref()
    }
    fn search_api(&self) -> &dyn search_api::SearchApi {
        self.search_api.as_ref()
    }
    fn selection_api(&self) -> &dyn selection_api::SelectionApi {
        self.selection_api.as_ref()
    }
    fn settings_api(&self) -> &dyn settings_api::SettingsApi {
        self.settings_api.as_ref()
    }
    fn stock_api(&self) -> &dyn stock_api::StockApi {
        self.stock_api.as_ref()
    }
    fn supplier_api(&self) -> &dyn supplier_api::SupplierApi {
        self.supplier_api.as_ref()
    }
    fn system_api(&self) -> &dyn system_api::SystemApi {
        self.system_api.as_ref()
    }
    fn system_internal_api(&self) -> &dyn system_internal_api::SystemInternalApi {
        self.system_internal_api.as_ref()
    }
    fn units_api(&self) -> &dyn units_api::UnitsApi {
        self.units_api.as_ref()
    }
    fn user_api(&self) -> &dyn user_api::UserApi {
        self.user_api.as_ref()
    }
    fn version_api(&self) -> &dyn version_api::VersionApi {
        self.version_api.as_ref()
    }
    fn version_text_api(&self) -> &dyn version_text_api::VersionTextApi {
        self.version_text_api.as_ref()
    }
    fn webhook_api(&self) -> &dyn webhook_api::WebhookApi {
        self.webhook_api.as_ref()
    }
}
