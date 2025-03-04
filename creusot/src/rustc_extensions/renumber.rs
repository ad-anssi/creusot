use rustc_index::vec::IndexVec;
use rustc_middle::mir::visit::{MutVisitor, TyContext};
use rustc_middle::mir::{Body, Location, PlaceElem, Promoted};
use rustc_middle::ty::subst::SubstsRef;
use rustc_middle::ty::{self, Ty, TyCtxt, TypeFoldable};
use rustc_trait_selection::infer::InferCtxt;
use rustc_trait_selection::infer::NllRegionVariableOrigin;

/// Replaces all free regions appearing in the MIR with fresh
/// inference variables, returning the number of variables created.
pub fn renumber_mir<'tcx>(
    infcx: &InferCtxt<'_, 'tcx>,
    body: &mut Body<'tcx>,
    promoted: &mut IndexVec<Promoted, Body<'tcx>>,
) {
    let mut visitor = NllVisitor { infcx };

    for body in promoted.iter_mut() {
        visitor.visit_body(body);
    }

    visitor.visit_body(body);
}

/// Replaces all regions appearing in `value` with fresh inference
/// variables.
pub fn renumber_regions<'tcx, T>(infcx: &InferCtxt<'_, 'tcx>, value: T) -> T
where
    T: TypeFoldable<'tcx>,
{
    infcx.tcx.fold_regions(value, &mut false, |_region, _depth| {
        let origin = NllRegionVariableOrigin::Existential { from_forall: false };
        infcx.next_nll_region_var(origin)
    })
}

struct NllVisitor<'a, 'tcx> {
    infcx: &'a InferCtxt<'a, 'tcx>,
}

impl<'a, 'tcx> NllVisitor<'a, 'tcx> {
    fn renumber_regions<T>(&mut self, value: T) -> T
    where
        T: TypeFoldable<'tcx>,
    {
        renumber_regions(self.infcx, value)
    }
}

impl<'a, 'tcx> MutVisitor<'tcx> for NllVisitor<'a, 'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.infcx.tcx
    }

    fn visit_ty(&mut self, ty: &mut Ty<'tcx>, _ty_context: TyContext) {
        *ty = self.renumber_regions(ty);
    }

    fn process_projection_elem(
        &mut self,
        elem: PlaceElem<'tcx>,
        _: Location,
    ) -> Option<PlaceElem<'tcx>> {
        if let PlaceElem::Field(field, ty) = elem {
            let new_ty = self.renumber_regions(ty);

            if new_ty != ty {
                return Some(PlaceElem::Field(field, new_ty));
            }
        }

        None
    }

    fn visit_substs(&mut self, substs: &mut SubstsRef<'tcx>, _location: Location) {
        *substs = self.renumber_regions(*substs);
    }

    fn visit_region(&mut self, region: &mut ty::Region<'tcx>, _location: Location) {
        let old_region = *region;
        *region = self.renumber_regions(&old_region);
    }

    fn visit_const(&mut self, constant: &mut &'tcx ty::Const<'tcx>, _location: Location) {
        *constant = self.renumber_regions(&*constant);
    }
}
