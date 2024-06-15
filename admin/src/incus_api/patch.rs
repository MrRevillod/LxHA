use std::ops::Deref;
use axum_responses::{AxumResult, HttpResponse};
use lxha_lib::app::constants::{INCUS_API, NET_INTERFACE};
use serde_json::Value;

use super::{
    get_client,
    patch_wrap
};

pub async fn update_profile(project: String) -> AxumResult<HttpResponse> {
    let client = get_client()?;

    let url = format!("{}/1.0/profiles/default?project={}", INCUS_API.deref(), project);

    let template_profile = format!("{{\"config\":{{\
                                          \"raw.qemu\":\"-enable-kvm\"\
                                       }},\
                                      \"devices\":{{\
                                          \"eth0\":{{\
                                              \"name\":\"eth0\",\
                                              \"nictype\":\"bridged\",\
                                              \"parent\":\"{}\",\
                                              \"type\":\"nic\"\
                                          }},\
                                          \"root\":{{\
                                              \"path\":\"/\",\
                                              \"pool\":\"incus-ceph-pool\",\
                                              \"type\":\"disk\"\
                                          }}\
                                       }}\
                                    }}",
                                    NET_INTERFACE.deref());

    let project_status: Value = patch_wrap(client.clone(), template_profile, url)
        .await?
        .json()
        .await
        .unwrap();

    println!("{:#?}", project_status);

    Ok(HttpResponse::OK)
}