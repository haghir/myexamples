mod foo;
mod bar;

use foo::foo1::foo1;
use foo::foofoo::foo2::foo2;
use foo::foofoo::foo3;
use foo::foo1a;

// Import the method bar1 with full path.
use crate::bar::bar1::bar1;

pub fn libfoo() {
    foo1("libfoo");
    foo1a("libfoo");
    foo2("libfoo");
    foo3("foo3");
    bar1("libfoo");
}