/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
)

type BarBfCfgInfo struct {
	Z int
}

type BarBfCfgSrc struct {
	Get func() BarBfCfgInfo
}

type BarBfT = func()

func BarBfC(cfg BarBfCfgSrc) BarBfT {
	return func() {
		fmt.Println(cfg.Get().Z)
	}
}
