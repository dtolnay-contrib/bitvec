/// Construct a `BitVec` out of a literal array in source code, analagous to
/// `vec!`.
///
/// `bitvec!` can be invoked in a number of ways. It takes the name of an
/// `Endian` implementation, the name of a `Bits`-implementing primitive, and
/// zero or more primitives (integer, floating-point, or bool) which are used to
/// build the bits. Each primitive literal corresponds to one bit, and is
/// considered to represent 1 if *any* bit in the representation is set.
///
/// `bitvec!` can be invoked with no specifiers, and `Endian` specifier, or an
/// `Endian` and a `Bits` specifier. It cannot be invoked with a `Bits`
/// specifier but no `Endian` specifier, due to overlap in how those tokens are
/// matched by the macro system.
///
/// Like `vec!`, `bitvec!` supports bit lists `[0, 1, ...]` and repetition
/// markers `[1; n]`.
///
/// # All Syntaxes
///
/// ```rust
/// # use bitvec::*;
/// bitvec![BigEndian, u8; 0, 1];
/// bitvec![LittleEndian, u8; 0, 1,];
/// bitvec![BigEndian; 0, 1];
/// bitvec![LittleEndian; 0, 1,];
/// bitvec![0, 1];
/// bitvec![0, 1,];
/// bitvec![BigEndian, u8; 1; 5];
/// bitvec![LittleEndian; 0; 5];
/// bitvec![1; 5];
/// ```
#[macro_export]
macro_rules! bitvec {
	//  bitvec![endian, type ; 0, 1, ...]
	( $end:ident , $prim:ty ; $( $elt:expr ),* ) => {
		__bitvec_impl![ $end, $prim ; $( $elt ),* ]
	};
	//  bitvec![endian, type ; 0, 1, ..., ]
	( $end:ident , $prim:ty ; $( $elt:expr , )* ) => {
		__bitvec_impl![ $end , $prim ; $( $elt ),* ]
	};
	//  bitvec![endian ; 0, 1, ...]
	( $end:ident ; $( $elt:expr ),* ) => {
		__bitvec_impl![ $end , u8 ; $( $elt ),* ]
	};
	//  bitvec![endian ; 0, 1, ..., ]
	( $end:ident ; $( $elt:expr , )* ) => {
		__bitvec_impl![ $end , u8 ; $( $elt ),* ]
	};
	//  bitvec![0, 1, ...]
	( $( $elt:expr ),* ) => {
		__bitvec_impl![ BigEndian , u8 ; $($elt),* ]
	};
	//  bitvec![0, 1, ..., ]
	( $( $elt:expr , )* ) => {
		__bitvec_impl![ BigEndian , u8 ; $($elt),* ]
	};

	//  bitvec![endian, type, bit; rep]
	( $end:ident , $prim:ty ; $elt:expr ; $rep:expr ) => {
		__bitvec_impl![ $end , $prim ; $elt; $rep ]
	};
	//  bitvec![endian, bit; rep]
	( $end:ident ; $elt:expr ; $rep:expr ) => {
		__bitvec_impl![ $end , u8 ;  $elt; $rep ]
	};
	//  bitvec![bit; rep]
	( $elt:expr ; $rep:expr ) => {
		__bitvec_impl![ BigEndian, u8 ; $elt; $rep ]
	};
}

/// Build an array of `bool` (one bit per byte) and then build a `BitVec` from that (one
/// bit per bit). I have yet to think of a way to make the source array be
/// binary-compatible with a `BitVec` representation, so the static source is 8x larger
/// than it needs to be.
///
/// I'm sure there is a way, but I don’t think I need to spend the effort yet.
#[macro_export]
#[doc(hidden)]
macro_rules! __bitvec_impl {
	( $end:ident , $prim:ty ; $( $elt:expr ),* ) => {{
		let init: &[bool] = &[
			$( $elt as u8 > 0 ),*
		];
		$crate :: BitVec ::< $crate :: $end , $prim >:: from(init)
	}};

	( $end:ident , $prim:ty ; $elt:expr; $rep:expr ) => {{
		::std::iter::repeat( $elt as u8 > 0 )
			.take( $rep )
			.collect ::< $crate :: BitVec < $crate :: $end , $prim > > ()
	}};
}

#[doc(hidden)]
macro_rules! __bitslice_shift {
	( $( $t:ty ),+ ) => { $(
#[doc(hidden)]
impl<E: $crate::Endian, T: $crate::Bits> ShlAssign< $t > for $crate::BitSlice<E, T> {
	fn shl_assign(&mut self, shamt: $t ) {
		ShlAssign::<usize>::shl_assign(self, shamt as usize);
	}
}

#[doc(hidden)]
impl<E: $crate::Endian, T: $crate::Bits> ShrAssign< $t > for $crate::BitSlice<E, T> {
	fn shr_assign(&mut self, shamt: $t ) {
		ShrAssign::<usize>::shr_assign(self, shamt as usize);
	}
}
	)+ };
}

#[doc(hidden)]
macro_rules! __bitvec_shift {
	( $( $t:ty ),+ ) => { $(
#[doc(hidden)]
impl<E: $crate::Endian, T: $crate::Bits> Shl< $t > for $crate::BitVec<E, T> {
	type Output = <Self as Shl<usize>>::Output;

	fn shl(self, shamt: $t ) -> Self::Output {
		Shl::<usize>::shl(self, shamt as usize)
	}
}

#[doc(hidden)]
impl<E: $crate::Endian, T: $crate::Bits> ShlAssign< $t > for $crate::BitVec<E, T> {
	fn shl_assign(&mut self, shamt: $t ) {
		ShlAssign::<usize>::shl_assign(self, shamt as usize)
	}
}

#[doc(hidden)]
impl<E: $crate::Endian, T: $crate::Bits> Shr< $t > for $crate::BitVec<E, T> {
	type Output = <Self as Shr<usize>>::Output;

	fn shr(self, shamt: $t ) -> Self::Output {
		Shr::<usize>::shr(self, shamt as usize)
	}
}

#[doc(hidden)]
impl<E: $crate::Endian, T: $crate::Bits> ShrAssign< $t > for $crate::BitVec<E, T> {
	fn shr_assign(&mut self, shamt: $t ) {
		ShrAssign::<usize>::shr_assign(self, shamt as usize)
	}
}
	)+ };
}

#[cfg(test)]
mod tests {
	#[test]
	fn compile_macros() {
		bitvec![0, 1];
		bitvec![BigEndian; 0, 1];
		bitvec![LittleEndian; 0, 1];
		bitvec![BigEndian, u8; 0, 1];
		bitvec![LittleEndian, u8; 0, 1];
		bitvec![BigEndian, u16; 0, 1];
		bitvec![LittleEndian, u16; 0, 1];
		bitvec![BigEndian, u32; 0, 1];
		bitvec![LittleEndian, u32; 0, 1];
		bitvec![BigEndian, u64; 0, 1];
		bitvec![LittleEndian, u64; 0, 1];

		bitvec![1; 70];
		bitvec![BigEndian; 0; 70];
		bitvec![LittleEndian; 1; 70];
		bitvec![BigEndian, u8; 0; 70];
		bitvec![LittleEndian, u8; 1; 70];
		bitvec![BigEndian, u16; 0; 70];
		bitvec![LittleEndian, u16; 1; 70];
		bitvec![BigEndian, u32; 0; 70];
		bitvec![LittleEndian, u32; 1; 70];
		bitvec![BigEndian, u64; 0; 70];
		bitvec![LittleEndian, u64; 1; 70];
	}
}