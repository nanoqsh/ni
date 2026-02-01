use {crate::Name, bytemuck::NoUninit};

// SAFETY: the `Name` type
// * is inhabited.
// * has no paddings.
// * content is `NoUninit` (implicitly).
// * is `repr(C)`
// * doesn't contain any interior mutability.
unsafe impl NoUninit for Name {}
