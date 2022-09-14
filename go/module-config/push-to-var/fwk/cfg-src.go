/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package fwk

type CfgSrc[T any] func() T

func NilCfgSrc[T any]() T {
	panic("Module used before being initialized")
}

type CfgSrcAdapter[S, T any] func(S) T

type CfgSrcAdaptation[S, T any] struct {
	targetSrc *func() T
	adapter   CfgSrcAdapter[S, T]
}

func (s CfgSrcAdaptation[S, T]) SetOrigin(originSrc CfgSrc[S]) {
	*s.targetSrc = func() T { return s.adapter(originSrc()) }
}

func MakeCfgSrcAdaptation[S, T any](
	targetSrc *func() T,
	adapter CfgSrcAdapter[S, T],
) CfgSrcAdaptation[S, T] {
	return CfgSrcAdaptation[S, T]{
		targetSrc: targetSrc,
		adapter:   adapter,
	}
}
