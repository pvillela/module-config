/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
)

type FooSflCfgInfo struct {
	X string
}

type FooSflCfgSrc struct {
	Get   func() FooSflCfgInfo
	BarBf BarBfT
}

type FooSflT = func()

func FooSflC(cfg FooSflCfgSrc) FooSflT {
	return func() {
		fmt.Println(cfg.Get().X)
		cfg.BarBf()
	}
}
