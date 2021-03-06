use syntax::ast;
use syntax::codemap::{DUMMY_SP};

use invoke::{Invoke, Identity};
use name::ToName;

//////////////////////////////////////////////////////////////////////////////

pub trait IntoLifetime {
    fn into_lifetime(self) -> ast::Lifetime;
}

impl IntoLifetime for ast::Lifetime {
    fn into_lifetime(self) -> ast::Lifetime {
        self
    }
}

impl<'a> IntoLifetime for &'a str {
    fn into_lifetime(self) -> ast::Lifetime {
        ast::Lifetime {
            id: ast::DUMMY_NODE_ID,
            span: DUMMY_SP,
            name: self.to_name(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

pub trait IntoLifetimeDef {
    fn into_lifetime_def(self) -> ast::LifetimeDef;
}

impl IntoLifetimeDef for ast::LifetimeDef {
    fn into_lifetime_def(self) -> ast::LifetimeDef {
        self
    }
}

impl IntoLifetimeDef for ast::Lifetime {
    fn into_lifetime_def(self) -> ast::LifetimeDef {
        ast::LifetimeDef {
            lifetime: self,
            bounds: vec![],
        }
    }
}

impl<'a> IntoLifetimeDef for &'a str {
    fn into_lifetime_def(self) -> ast::LifetimeDef {
        self.into_lifetime().into_lifetime_def()
    }
}

impl IntoLifetimeDef for String {
    fn into_lifetime_def(self) -> ast::LifetimeDef {
        (*self).into_lifetime().into_lifetime_def()
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct LifetimeDefBuilder<F=Identity> {
    callback: F,
    lifetime: ast::Lifetime,
    bounds: Vec<ast::Lifetime>,
}

impl LifetimeDefBuilder {
    pub fn new<N>(name: N) -> Self
        where N: ToName,
    {
        LifetimeDefBuilder::with_callback(name, Identity)
    }
}

impl<F> LifetimeDefBuilder<F>
    where F: Invoke<ast::LifetimeDef>,
{
    pub fn with_callback<N>(name: N, callback: F) -> Self
        where N: ToName,
    {
        let lifetime = ast::Lifetime {
            id: ast::DUMMY_NODE_ID,
            span: DUMMY_SP,
            name: name.to_name(),
        };

        LifetimeDefBuilder {
            callback: callback,
            lifetime: lifetime,
            bounds: Vec::new(),
        }
    }

    pub fn bound<N>(mut self, name: N) -> Self
        where N: ToName,
    {
        let lifetime = ast::Lifetime {
            id: ast::DUMMY_NODE_ID,
            span: DUMMY_SP,
            name: name.to_name(),
        };

        self.bounds.push(lifetime);
        self
    }

    pub fn build(self) -> F::Result {
        self.callback.invoke(ast::LifetimeDef {
            lifetime: self.lifetime,
            bounds: self.bounds,
        })
    }
}
