//! `vendor` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::{
    DuplicateVendorKeyException, ModelError, Person, PersonCreatedEvent, PersonId, PersonName,
    PersonNameChangedEvent, PersonNotFoundException, VendorCreatedEvent, VendorId, VendorKey,
    VendorName, VendorRef,
};
use ddd_4_rust_core::{
    AggregateError, AggregateRoot, AggregateVersion, DomainEvent, Entity, EntityId, EntityType,
};
use thiserror::Error;

/// Service invoked before a vendor key is accepted.
///
/// `ConstructorService` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait ConstructorService {
    /// Persists or rejects a unique vendor key.
    ///
    /// 执行 `add_vendor_key` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn add_vendor_key(&mut self, key: &VendorKey) -> Result<(), DuplicateVendorKeyException>;
}
impl<F> ConstructorService for F
where
    F: FnMut(&VendorKey) -> Result<(), DuplicateVendorKeyException>,
{
    /// 执行 `add_vendor_key` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn add_vendor_key(&mut self, key: &VendorKey) -> Result<(), DuplicateVendorKeyException> {
        self(key)
    }
}

/// Failure produced by executable vendor operations.
#[derive(Debug, Error)]
/// `VendorError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum VendorError {
    /// Duplicate business key.
    #[error(transparent)]
    DuplicateKey(#[from] DuplicateVendorKeyException),
    /// Invalid event construction.
    #[error(transparent)]
    Model(#[from] ModelError),
    /// Missing child entity.
    #[error(transparent)]
    PersonNotFound(#[from] PersonNotFoundException),
}

/// Vendor aggregate used by all serializer compatibility suites.
#[expect(
    clippy::struct_field_names,
    reason = "VendorRef is the Java field name"
)]
/// `Vendor` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct Vendor {
    vendor_ref: VendorRef,
    last_person_id: i32,
    persons: Vec<Person>,
    version: i32,
    changes: Vec<Box<dyn DomainEvent<dyn EntityId>>>,
}

impl Vendor {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(
        id: VendorId,
        key: VendorKey,
        name: VendorName,
        service: &mut dyn ConstructorService,
    ) -> Result<Self, VendorError> {
        service.add_vendor_key(&key)?;
        let vendor_ref = VendorRef::new(id, key, name);
        let created = VendorCreatedEvent::new(vendor_ref.clone())?;
        Ok(Self {
            vendor_ref,
            last_person_id: 0,
            persons: Vec::new(),
            version: -1,
            changes: vec![Box::new(created)],
        })
    }
    #[must_use]
    /// 执行 `vendor_ref` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn vendor_ref(&self) -> &VendorRef {
        &self.vendor_ref
    }
    #[must_use]
    /// 执行 `persons` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn persons(&self) -> &[Person] {
        &self.persons
    }
    /// 执行 `add_person` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    pub fn add_person(&mut self, name: PersonName) -> Result<PersonId, VendorError> {
        self.last_person_id = self.last_person_id.saturating_add(1);
        let id = PersonId::new(self.last_person_id);
        let event = PersonCreatedEvent::new(self.vendor_ref.clone(), id, name.clone(), self.next_apply_version())?;
        self.persons.push(Person::new(id, name));
        self.changes.push(Box::new(event));
        Ok(id)
    }
    /// 执行 `change_person_name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn change_person_name(
        &mut self,
        person_id: PersonId,
        new_name: PersonName,
    ) -> Result<(), VendorError> {
        let version = self.next_apply_version();
        let Some(person) = self
            .persons
            .iter_mut()
            .find(|person| person.id() == person_id)
        else {
            return Err(PersonNotFoundException::new(self.vendor_ref.clone(), person_id).into());
        };
        let event: PersonNameChangedEvent =
            person.change_name(self.vendor_ref.clone(), new_name, version)?;
        self.changes.push(Box::new(event));
        Ok(())
    }
}

impl Entity<VendorId> for Vendor {
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType {
        self.vendor_ref.id().entity_type()
    }
    /// 执行 `id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn id(&self) -> &VendorId {
        self.vendor_ref.id()
    }
}
impl AggregateRoot<VendorId> for Vendor {
    /// 执行 `uncommitted_changes` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn uncommitted_changes(&self) -> &[Box<dyn DomainEvent<dyn EntityId>>] {
        &self.changes
    }
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn mark_changes_as_committed(&mut self) {
        self.version = self.next_version();
        self.changes.clear();
    }
    /// 执行 `version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn version(&self) -> i32 {
        self.version
    }
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn load_from_history(
        &mut self,
        history: &[Box<dyn DomainEvent<dyn EntityId>>],
    ) -> Result<(), AggregateError> {
        let count = i32::try_from(history.len()).unwrap_or(i32::MAX);
        self.version = self.version.saturating_add(count);
        Ok(())
    }
    /// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
    /// 该方法会修改接收者状态；调用完成后仍需满足类型不变式。
    fn apply(&mut self, event: Box<dyn DomainEvent<dyn EntityId>>) -> Result<(), AggregateError> {
        self.changes.push(event);
        Ok(())
    }
}

impl Vendor {
    /// Current aggregate version value for serializer fixtures.
    #[must_use]
    /// 执行 `aggregate_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn aggregate_version(&self) -> AggregateVersion {
        self.next_apply_version()
    }
}
