// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use anyhow::{bail, Result};
use chrono::offset::LocalResult;
use chrono::{TimeZone, Utc};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tonic::Request;

use super::pb::{ExportDataReq, HeartbeatReq, TaskDoneReq, Unit};
use crate::config::RpcConfig;
use crate::runtime::Task;
use crate::services::host_info::RawInfo;
use crate::services::pb::psh_service_client::PshServiceClient;
use crate::services::pb::{GetTaskReq, SendHostInfoReq};

#[derive(Clone)]
pub struct RpcClient {
    token: String,
    client: PshServiceClient<Channel>,
    instance_id_file: String,
}

impl RpcClient {
    pub async fn new(config: &RpcConfig, token: String) -> Result<Self> {
        let ep = Endpoint::from_shared(config.addr.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?;
        let client: PshServiceClient<Channel> = PshServiceClient::connect(ep).await?;
        Ok(Self {
            token,
            client,
            instance_id_file: config.instance_id_file.clone(),
        })
    }

    pub fn instance_id(&self) -> Result<String> {
        Ok(std::fs::read_to_string(&self.instance_id_file)?)
    }

    pub async fn send_host_info(&mut self, instance_id: String) -> Result<()> {
        let req = {
            let raw_info = RawInfo::new();
            let req = SendHostInfoReq {
                instance_id,
                os: raw_info.os,
                architecture: raw_info.arch,
                hostname: raw_info.hostname,
                kernel_version: raw_info.kernel_version,
                local_ipv4_addr: raw_info.ipv4.map(|it| it.to_bits().to_be()),
                local_ipv6_addr: raw_info.ipv6.map(|it| it.into()),
            };
            let mut req = Request::new(req);
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        let resp = self.client.send_host_info(req).await?;

        let resp = resp.get_ref();

        tracing::trace!("{:?}", resp);

        Ok(())
    }

    pub async fn export_data(&mut self, req: ExportDataReq) -> Result<()> {
        let mut req = Request::new(req);
        req.metadata_mut()
            .insert("authorization", format!("Bearer {}", self.token).parse()?);
        self.client.export_data(req).await?;
        Ok(())
    }

    pub async fn heartbeat(&mut self, req: HeartbeatReq) -> Result<()> {
        let req = {
            let mut req = Request::new(req);
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        self.client.heartbeat(req).await?;

        Ok(())
    }

    pub async fn get_task(&mut self, instance_id: String) -> Result<Option<Task>> {
        let req = {
            let mut req = Request::new(GetTaskReq { instance_id });
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        let Some(task) = self.client.get_task(req).await?.into_inner().task else {
            return Ok(None);
        };

        let end_time = match Utc.timestamp_millis_opt(task.end_time as _) {
            LocalResult::Single(t) => t,
            _ => bail!("Invalid task end time"),
        };
        let task = Task {
            id: Some(task.id),
            wasm_component: task.wasm,
            wasm_component_args: task.wasm_args,
            end_time,
        };

        Ok(Some(task))
    }

    pub async fn task_done(&mut self, task_id: String) -> Result<()> {
        let req = {
            let mut req = Request::new(TaskDoneReq { task_id });
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        self.client.task_done(req).await?;

        Ok(())
    }

    pub async fn new_instance_id(&mut self) -> Result<String> {
        let req = {
            let mut req = Request::new(Unit {});
            req.metadata_mut()
                .insert("authorization", format!("Bearer {}", self.token).parse()?);
            req
        };

        let resp = self.client.new_instance_id(req).await?;

        Ok(resp.into_inner().instance_id)
    }
}
