use deno_core::error::AnyError;
use crate::{graphql::graphql_types::{OnlineAgent, PerspectiveExpression}, js_core::JsCoreHandle, types::{Perspective, PerspectiveDiff}};
use super::byte_array::ByteArray;

#[derive(Clone)]
pub struct Language {
    address: String,
    js_core: JsCoreHandle,
}

fn parse_revision(js_result: String) -> Result<Option<String>, AnyError> {
    if let Ok(maybe_revision) = serde_json::from_str::<Option<ByteArray>>(&js_result) {
        Ok(maybe_revision.map(|revision| {
            let vec: Vec<u8> = revision.into();
            base64::encode(&vec)
        }))
    } else {
        Ok(serde_json::from_str::<Option<String>>(&js_result)?)
    }
}
impl Language {
    pub fn new(address: String, js_core: JsCoreHandle) -> Self {
        Self {
            address,
            js_core
        }
    }

    pub async fn sync(&mut self) -> Result<(), AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.linksAdapter.sync()
                    ?? null
                )
            "#,
            self.address,
        );
        let _result: String = self.js_core.execute(script).await?;
        Ok(())
    }

    pub async fn commit(&mut self, diff: PerspectiveDiff) -> Result<Option<String>, AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.linksAdapter.commit({})
                    ?? null
                )
            "#,
            self.address,
            serde_json::to_string(&diff)?,
        );
        let result: String = self.js_core.execute(script).await?;
        parse_revision(result)
    }

    pub async fn current_revision(&mut self) -> Result<Option<String>, AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.linksAdapter.currentRevision()
                    ?? null
                )
            "#,
            self.address,
        );
        let result: String = self.js_core.execute(script).await?;
        parse_revision(result)
    }

    pub async fn render(&mut self) -> Result<Option<Perspective>, AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.linksAdapter.render()
                    ?? null
                )
            "#,
            self.address,
        );
        let result: String = self.js_core.execute(script).await?;
        let maybe_value = serde_json::from_str(&result)?;
        Ok(maybe_value)
    }

    pub async fn others(&mut self) -> Result<Vec<String>, AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.linksAdapter.others()
                    ?? null
                )
            "#,
            self.address,
        );
        let result: String = self.js_core.execute(script).await?;
        let others_vec = serde_json::from_str(&result)?;
        Ok(others_vec)
    }

    pub async fn has_telepresence_adapter(&mut self) -> Result<bool, AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    !!(await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.telepresenceAdapter)
                )
            "#,
            self.address,
        );
        let result: String = self.js_core.execute(script).await?;
        let has_telepresence_adapter = serde_json::from_str(&result)?;
        Ok(has_telepresence_adapter)
    }


    pub async fn set_online_status(&mut self, status: PerspectiveExpression) -> Result<(), AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.telepresenceAdapter.setOnlineStatus({})
                    ?? null
                )
            "#,
            self.address,
            serde_json::to_string(&status)?,
        );
        let _result: String = self.js_core.execute(script).await?;
        Ok(())
    }

    pub async fn get_online_agents(&mut self) -> Result<Vec<OnlineAgent>, AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.telepresenceAdapter.getOnlineAgents()
                    ?? null
                )
            "#,
            self.address,
        );
        let result: String = self.js_core.execute(script).await?;
        let online_agents = serde_json::from_str(&result)?;
        Ok(online_agents)
    }

    pub async fn send_signal(
        &mut self,
        remote_agent_did: String,
        payload: PerspectiveExpression
    ) -> Result<(), AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.telepresenceAdapter.sendSignal("{}", {})
                    ?? null
                )
            "#,
            self.address,
            remote_agent_did,
            serde_json::to_string(&payload)?,
        );
        let _result: String = self.js_core.execute(script).await?;
        Ok(())
    }

    pub async fn send_broadcast(
        &mut self,
        payload: PerspectiveExpression
    ) -> Result<(), AnyError> {
        let script = format!(
            r#"
                JSON.stringify(
                    await (await core.languageController.languageByRef({{address:"{}"}}))
                        ?.telepresenceAdapter.sendBroadcast({})
                    ?? null
                )
            "#,
            self.address,
            serde_json::to_string(&payload)?,
        );
        let _result: String = self.js_core.execute(script).await?;
        Ok(())
    }
}
