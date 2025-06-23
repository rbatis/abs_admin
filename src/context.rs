use std::sync::{Arc, LazyLock};
use std::time::Duration;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use crate::config::config::ApplicationConfig;
use crate::service::{CacheService, StorageService, SysAuthService, SysDictService, RbacPermissionService, RbacRolePermissionService, RbacRoleService, SysTrashService, RbacUserRoleService, SysUserService};

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
    pub storage_service: StorageService,
    pub sys_user_service: SysUserService,
    pub rbac_permission_service: RbacPermissionService,
    pub rbac_role_service: RbacRoleService,
    pub rbac_role_permission_service: RbacRolePermissionService,
    pub rbac_user_role_service: RbacUserRoleService,
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
        //level
        self.rb.get_intercept::<LogInterceptor>().expect("rbatis LogInterceptor init fail!").set_level_filter(log::max_level());
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
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: {
                let rb = RBatis::new();
                rb
            },
            cache_service: CacheService::new(&config).unwrap(),
            storage_service:  StorageService::new(&config.storage).expect("Failed to create storage service"),
            sys_user_service: SysUserService {},
            rbac_role_service: RbacRoleService {},
            rbac_permission_service: RbacPermissionService {},
            rbac_role_permission_service: RbacRolePermissionService {},
            rbac_user_role_service: RbacUserRoleService {},
            sys_dict_service: SysDictService {},
            sys_auth_service: SysAuthService {},
            sys_trash_service: SysTrashService::new(),
            config,
        }
    }
}
