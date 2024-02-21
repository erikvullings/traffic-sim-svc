/// A start inject request is a specific request for starting a pre-defined sequence of events defined at one or more connected applications. *Copyright (C) 2019-2020 XVR Simulation B.V., Delft, The Netherlands, Martijn Hendriks <hendriks @ xvrsim.com>. This file is licensed under the MIT license : https://github.com/DRIVER-EU/avro-schemas/blob/master/LICENSE*
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct RequestStartInject {
    pub id: String,
    pub applicant: String,
    pub inject: String,
    #[serde(default = "default_requeststartinject_tags")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[inline(always)]
fn default_requeststartinject_tags() -> Option<::std::collections::HashMap<String, String>> {
    None
}
