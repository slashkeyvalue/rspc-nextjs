use rspc::RouterBuilder;

use super::{PrivateCtx, PrivateRouter};

pub(crate) fn private_route() -> RouterBuilder<PrivateCtx> {
    PrivateRouter::new()
        .query("get", |t| {
            t(|ctx, _: ()| async move {

                Ok("user_id_placeholder")
            })
        })
}
