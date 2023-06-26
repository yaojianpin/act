use crate::{
    sch::Scheduler,
    store::{Store, StoreAdapter},
    utils::Id,
    ActInfo, ActResult, ActionState, ModelInfo, ProcInfo, Query, TaskInfo, Workflow,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Manager {
    scher: Arc<Scheduler>,
    store: Arc<Store>,
}

impl Manager {
    pub(crate) fn new(sch: &Arc<Scheduler>, store: &Arc<Store>) -> Self {
        Self {
            scher: sch.clone(),
            store: store.clone(),
        }
    }

    pub fn deploy(&self, model: &Workflow) -> ActResult<ActionState> {
        let mut state = ActionState::begin();
        self.store.deploy(model)?;
        state.end();

        Ok(state)
    }

    pub fn models(&self, limit: usize) -> ActResult<Vec<ModelInfo>> {
        let query = Query::new().set_limit(limit);
        match self.store.models().query(&query) {
            Ok(models) => {
                let mut ret = Vec::new();
                for m in models {
                    ret.push(m.into());
                }

                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }

    pub fn model(&self, id: &str) -> ActResult<ModelInfo> {
        match self.store.models().find(id) {
            Ok(m) => Ok(m.into()),
            Err(err) => Err(err),
        }
    }

    pub fn remove(&self, model_id: &str) -> ActResult<bool> {
        self.store.models().delete(model_id)
    }

    pub fn procs(&self, cap: usize) -> ActResult<Vec<ProcInfo>> {
        let query = Query::new().set_limit(cap);
        match self.store.procs().query(&query) {
            Ok(ref procs) => {
                let mut ret = Vec::new();
                for t in procs {
                    ret.push(t.into());
                }

                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }

    pub fn proc(&self, pid: &str) -> ActResult<ProcInfo> {
        match self.store.procs().find(pid) {
            Ok(ref proc) => Ok(proc.into()),
            Err(err) => Err(err),
        }
    }

    pub fn tasks(&self, pid: &str) -> ActResult<Vec<TaskInfo>> {
        let query = Query::new().push("pid", pid);
        match self.store.tasks().query(&query) {
            Ok(tasks) => {
                let mut ret = Vec::new();
                for t in tasks {
                    ret.push(t.into());
                }

                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }

    pub fn task(&self, pid: &str, tid: &str) -> ActResult<TaskInfo> {
        let id = Id::new(pid, tid);
        match self.store.tasks().find(&id.id()) {
            Ok(t) => Ok(t.into()),
            Err(err) => Err(err),
        }
    }

    pub fn acts(&self, pid: &str, tid: Option<&str>) -> ActResult<Vec<ActInfo>> {
        let mut query = Query::new().push("pid", pid);
        if let Some(tid) = tid {
            query = query.push("tid", tid);
        }
        match self.store.acts().query(&query) {
            Ok(acts) => {
                let mut ret = Vec::new();
                for t in acts {
                    ret.push(t.into());
                }

                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }

    pub fn close(&self, pid: &str) -> ActResult<bool> {
        self.scher.cache().remove(pid)
    }
}
