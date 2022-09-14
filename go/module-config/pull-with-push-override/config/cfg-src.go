/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package config

type CfgSrc[T any] func() T

func nilCfgSrc[T any]() T {
	panic("Module used before being initialized")
}

func MakeCfgSrc[T any](adapter func(src AppCfgInfo) T) CfgSrc[T] {
	if adapter == nil {
		return nilCfgSrc[T]
	}
	return func() T { return adapter(GetAppConfiguration()) }
}
