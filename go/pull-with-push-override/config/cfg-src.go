/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
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
