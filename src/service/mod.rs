mod res_service;

use res_service::ResService;

lazy_static! {
   pub static ref RES_SERVICE:ResService=ResService{};
}
