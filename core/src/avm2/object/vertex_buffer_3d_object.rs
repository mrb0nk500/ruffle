//! Object representation for VertexBuffer3D objects

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::barrier::unlock;
use gc_arena::lock::RefLock;
use gc_arena::{Collect, Gc, GcWeak, Mutation};
use ruffle_render::backend::VertexBuffer;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

use super::Context3DObject;

#[derive(Clone, Collect, Copy)]
#[collect(no_drop)]
pub struct VertexBuffer3DObject<'gc>(pub Gc<'gc, VertexBuffer3DObjectData<'gc>>);

#[derive(Clone, Collect, Copy, Debug)]
#[collect(no_drop)]
pub struct VertexBuffer3DObjectWeak<'gc>(pub GcWeak<'gc, VertexBuffer3DObjectData<'gc>>);

impl<'gc> VertexBuffer3DObject<'gc> {
    pub fn from_handle(
        activation: &mut Activation<'_, 'gc>,
        context3d: Context3DObject<'gc>,
        handle: Rc<dyn VertexBuffer>,
        data32_per_vertex: u8,
    ) -> Result<Object<'gc>, Error<'gc>> {
        let class = activation.avm2().classes().vertexbuffer3d;

        let mut this: Object<'gc> = VertexBuffer3DObject(Gc::new(
            activation.gc(),
            VertexBuffer3DObjectData {
                base: RefLock::new(ScriptObjectData::new(class)),
                context3d,
                handle,
                data32_per_vertex,
            },
        ))
        .into();
        this.install_instance_slots(activation.gc());

        class.call_native_init(this.into(), &[], activation)?;

        Ok(this)
    }

    pub fn handle(&self) -> Rc<dyn VertexBuffer> {
        self.0.handle.clone()
    }

    pub fn context3d(&self) -> Context3DObject<'gc> {
        self.0.context3d
    }

    pub fn data32_per_vertex(&self) -> u8 {
        self.0.data32_per_vertex
    }
}

#[derive(Collect)]
#[collect(no_drop)]
pub struct VertexBuffer3DObjectData<'gc> {
    /// Base script object
    base: RefLock<ScriptObjectData<'gc>>,

    context3d: Context3DObject<'gc>,

    #[collect(require_static)]
    handle: Rc<dyn VertexBuffer>,

    /// The 'data32PerVertex' value that this object was created with.
    /// This is the number of 32-bit values associated with each vertex,
    /// and is at most 64
    data32_per_vertex: u8,
}

impl<'gc> TObject<'gc> for VertexBuffer3DObject<'gc> {
    fn base(&self) -> Ref<ScriptObjectData<'gc>> {
        self.0.base.borrow()
    }

    fn base_mut(&self, mc: &Mutation<'gc>) -> RefMut<ScriptObjectData<'gc>> {
        unlock!(Gc::write(mc, self.0), VertexBuffer3DObjectData, base).borrow_mut()
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        Gc::as_ptr(self.0) as *const ObjectPtr
    }

    fn value_of(&self, _mc: &Mutation<'gc>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_vertex_buffer(&self) -> Option<VertexBuffer3DObject<'gc>> {
        Some(*self)
    }
}

impl std::fmt::Debug for VertexBuffer3DObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexBuffer3D")
    }
}
