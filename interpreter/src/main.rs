#![feature(rustc_private)]

use rustc_middle::hir::nested_filter;

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_macros;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

struct MyError {
    span: rustc_span::Span,
}

impl<'a> rustc_errors::Diagnostic<'a> for MyError {
    fn into_diag(
        self,
        dcx: rustc_errors::DiagCtxtHandle<'a>,
        level: rustc_errors::Level,
    ) -> rustc_errors::Diag<'a, rustc_span::ErrorGuaranteed> {
        rustc_errors::Diag::new(dcx, level, "my error message")
            .with_span(self.span)
            .with_note("hello leo")
    }
}

struct ProtoHal;

impl rustc_driver::Callbacks for ProtoHal {
    fn config(&mut self, _config: &mut rustc_interface::interface::Config) {}

    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let mut visitor = Visitor { tcx };

        tcx.hir_visit_all_item_likes_in_crate(&mut visitor);

        rustc_driver::Compilation::Stop
    }
}

struct Visitor<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
}

impl<'a> rustc_hir::intravisit::Visitor<'a> for Visitor<'a> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_local(&mut self, l: &'a rustc_hir::LetStmt<'a>) {
        self.tcx.sess.dcx().emit_err(MyError { span: l.span });
        let x = self.tcx.type_of(l.hir_id.owner).instantiate_identity();
        println!("{x:?}");
    }
}

fn main() {
    rustc_driver::install_ctrlc_handler();
    rustc_driver::run_compiler(
        std::env::args().collect::<Vec<_>>().as_slice(),
        &mut ProtoHal,
    );
}
