/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package startup

import (
	"github.com/pvillela/module-config/go/module-config/push-to-var/config"
	"github.com/pvillela/module-config/go/module-config/push-to-var/fs/cfgadapt"
)

func Initialize() struct{} {
	c := config.GetAppConfiguration
	cfgadapt.FooSflCfgAdaptation.SetOrigin(c)
	cfgadapt.BarBfCfgAdaptation.SetOrigin(c)
	return struct{}{}
}

var _ = Initialize()
