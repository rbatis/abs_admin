use std::sync::{Arc, LazyLock};
use std::time::Duration;
use rbatis::RBatis;
use crate::config::config::ApplicationConfig;
use crate::service::{CacheService, SysAuthService, SysDictService, SysPermissionService, SysRoleResService, SysRoleService, SysTrashService, SysUserRoleService, SysUserService};

/// Service CONTEXT
pub static CONTEXT: LazyLock<ServiceContext> = LazyLock::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::context::CONTEXT.rb
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub cache_service: CacheService,
    pub sys_permission_service: SysPermissionService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_permission_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_service: SysDictService,
    pub sys_auth_service: SysAuthService,
    pub sys_trash_service: SysTrashService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_database(&self) {
        log::info!("[abs_admin] rbatis pool init ({})...", self.config.db_url);
        //include auto choose driver struct by 'config.db_url'
        self.rb
            .link(include!("../target/driver.rs"), &self.config.db_url)
            .await
            .expect("[abs_admin] rbatis pool init fail!");
        self.rb.intercepts.push(Arc::new(SysTrashService::new()));
        let pool = self.rb.get_pool().unwrap();
        //max connections
        pool.set_max_open_conns(self.config.db_pool_len as u64)
            .await;
        //max timeout
        pool.set_timeout(Some(Duration::from_secs(
            self.config.db_pool_timeout as u64,
        )))
            .await;
        log::info!(
            "[abs_admin] rbatis pool init success! pool state = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
        log::info!(
            "Serve:   http://{}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let mut config = ApplicationConfig::default();
        ServiceContext {
            rb: {
                let rb = RBatis::new();
                if cfg!(debug_assertions) == false && config.debug.eq(&true) {
                    config.debug = false;
                }
                rb
            },
            cache_service: CacheService::new(&config).unwrap(),
            sys_permission_service: SysPermissionService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_permission_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_service: SysDictService {},
            sys_auth_service: SysAuthService {},
            sys_trash_service: SysTrashService::new(),
            config,
        }
    }
}