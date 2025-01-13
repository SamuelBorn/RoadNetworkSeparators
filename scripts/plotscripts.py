#!/usr/bin/env python3

import math
import sys

markers = ['^', 'v', 'x']
colors = ['#377eb8', '#ff7f00', '#e41a1c', '#f781bf', '#a65628', '#4daf4a', '#984ea3', '#999999', '#dede00', '#377eb8']

lim = 3600
out = 4000
border_lo = 0.001
border_hi = 5000

msize = 6
pltxsize = 5
pltysize = 5
xmin = None
xmax = None
ymin = None
ymax = None
y2 = None

files = []
runtime_maps = []
labels = []
xlabel = None
ylabel = None
min_val = lim
heading = ""
outfile = None
logscale = False
domainlabels = None
domainmarkers = None
domaincolors = None
tickslist = None
tickslabellist = None
nolegend = False
legendright = False
legendbottom = False
legend_columns = 1
legend_offset_x = 0
legend_offset_y = 0
legend_spacing = 0.5
outfile_legend = None
powers_of_ten_ticks = False
sansfont = False
timesfont = False
minorlogticks = False
lloc = None
topsymbols = None
toptexts = None

for arg in sys.argv[1:]:
    if arg.startswith("-l="):
        labels += [arg[3:]]
    elif arg.startswith("-h="):
        heading = arg[3:]
    elif arg.startswith("-size="):
        pltxsize = float(arg[6:])
        pltysize = float(arg[6:])
    elif arg.startswith("-xsize="):
        pltxsize = float(arg[7:])
    elif arg.startswith("-ysize="):
        pltysize = float(arg[7:])
    elif arg.startswith("-min="):
        border_lo = float(arg[5:])
        min_val = border_lo
    elif arg.startswith("-max="):
        border_hi = float(arg[5:])
    elif arg.startswith("-xlabel="):
        xlabel = arg[8:]
    elif arg.startswith("-ylabel="):
        ylabel = arg[8:]
    elif arg.startswith("-T="):
        lim = float(arg[3:])
        #out = lim * 1.35
        #border_hi = lim * 1.8
        #min_val = lim
    elif arg.startswith("-y2="):
        y2 = float(arg[4:])
    elif arg.startswith("-nolegend"):
        nolegend = True
    elif arg.startswith("-legend-cols="):
        legend_columns = int(arg[len("-legend-cols="):])
    elif arg.startswith("-legend-offset-x="):
        legend_offset_x = float(arg[len("-legend-offset-x="):])
    elif arg.startswith("-legend-offset-y="):
        legend_offset_y = float(arg[len("-legend-offset-y="):])
    elif arg.startswith("-legend-spacing="):
        legend_spacing = float(arg[len("-legend-spacing="):])
    elif arg.startswith("-lloc="):
        lloc = int(arg[len("-lloc="):])
    elif arg.startswith("-o="):
        outfile = arg[3:]
    elif arg.startswith("-ol="):
        outfile_legend = arg[len("-ol="):]
    elif arg.startswith("-logscale"):
        logscale = True
    elif arg.startswith("-legendright") or arg.startswith("-legend-right"):
        legendright = True
    elif arg.startswith("-legendbot") or arg.startswith("-legend-bot"):
        legendbottom = True
    elif arg.startswith("-domainlabels="):
        domainlabels = arg[len("-domainlabels="):].split(",")
    elif arg.startswith("-domainmarkers="):
        domainmarkers = arg[len("-domainmarkers="):].split(",")
    elif arg.startswith("-domaincolors="):
        domaincolors = arg[len("-domaincolors="):].split(",")
    elif arg.startswith("-topsymbols="):
        topsymbols = arg[len("-topsymbols="):]
    elif arg.startswith("-toptexts="):
        toptexts = arg[len("-toptexts="):]
    elif arg.startswith("-ticks="):
        tickslist = arg[len("-ticks="):].split(",")
    elif arg.startswith("-tickslabels="):
        tickslabellist = arg[len("-tickslabels="):].split(",")
    elif arg.startswith("-markersize="):
        msize = float(arg[len("-markersize="):])
    elif arg.startswith("-potticks"):
        powers_of_ten_ticks = True
    elif arg.startswith("-sansfont"):
        sansfont = True
    elif arg.startswith("-minorlogticks"):
        minorlogticks = True
    else:
        files += [arg]

import matplotlib
if outfile:
    matplotlib.use('pdf')
matplotlib.rcParams['hatch.linewidth'] = 0.5  # previous pdf hatch linewidth
import matplotlib.pyplot as plt
from matplotlib import rc
from matplotlib import ticker

rc('text', usetex=True)
if sansfont:
    matplotlib.rcParams['text.latex.preamble'] += r'\usepackage[cm]{sfmath}'
    #matplotlib.rcParams['font.family'] = 'sans-serif'
    #matplotlib.rcParams['font.sans-serif'] = 'cm'
    rc('font',**{'family':'sans-serif','sans-serif':['Helvetica']})
    #\renewcommand\familydefault{\sfdefault} 
else:
    rc('font', family='serif')
    if timesfont:
        rc('font', serif=['Times'])

out = math.exp((math.log(lim) + math.log(border_hi)) / 2)

runtime_map_pairs_by_domain = dict()
domain_to_style_map = dict()

for arg in files:
    domains_seen = set()
    for line in open(arg, 'r').readlines():
        words = line.rstrip().split(" ")
        id = words[0]
        dom = words[1]
        val = float(words[2])
        
        if dom[0].islower():
            dom = dom[0].upper() + dom[1:]
        
        if dom not in runtime_map_pairs_by_domain:
            runtime_map_pairs_by_domain[dom] = []
        if dom not in domains_seen:
            runtime_map_pairs_by_domain[dom] += [dict()]
            domains_seen.add(dom)
        id_runtime_map = runtime_map_pairs_by_domain[dom][-1]
        if val <= lim:
            id_runtime_map[id] = val
            #if val > 0:
            #    min_val = min(min_val, val)

for dom in runtime_map_pairs_by_domain:
    #if len(runtime_map_pairs_by_domain[dom]) == 1:
    #    runtime_map_pairs_by_domain[dom] += [dict()]
    if len(runtime_map_pairs_by_domain[dom]) != 2:
        print(f"Need exactly two runtime files for domain {dom}!")
        exit(1)

margin = lim - out

fig, ax = plt.subplots(1, 1, figsize=(pltxsize, pltysize))
ax.set_box_aspect(1)

plt.ylim(min_val, border_hi)
plt.xlim(min_val, border_hi)
if logscale:
    plt.xscale("log")
    plt.yscale("log")

if powers_of_ten_ticks:
    power = -7
    while 10**power < min_val:
        power += 1
    tickpos = []
    ticklabel = []
    while 10**power <= border_hi:
        tickpos += [10**power]
        ticklabel += ["$10^{"+str(power)+"}$"]
        power += 1
    ax.set_xticks(tickpos)
    ax.set_xticklabels(ticklabel)
    ax.set_yticks(tickpos)
    ax.set_yticklabels(ticklabel)
    #plt.minorticks_off()
    print(f"potticks {tickpos} {ticklabel}")
elif tickslist:
    ax.set_xticklabels(tickslabellist if tickslabellist else tickslist)
    ax.set_xticks([float(x) for x in tickslist])
    ax.set_yticklabels(tickslabellist if tickslabellist else tickslist)
    ax.set_yticks([float(x) for x in tickslist])
    plt.minorticks_off()

if minorlogticks:
    if not powers_of_ten_ticks:
        ax.xaxis.set_major_locator(ticker.LogLocator(base=10.0, subs=(1,), numticks=999))
        ax.yaxis.set_major_locator(ticker.LogLocator(base=10.0, subs=(1,), numticks=999))
    ax.xaxis.set_minor_locator(ticker.LogLocator(base=10.0, subs='all', numticks=999))
    ax.yaxis.set_minor_locator(ticker.LogLocator(base=10.0, subs='all', numticks=999))

plt.grid(color='#dddddd', linestyle='-', linewidth=1)

plt.plot([border_lo, lim], [border_lo, lim], 'black', alpha=0.3, linestyle="--")
#plt.plot([border_lo, border_hi], [10*border_lo, 10*border_hi], 'gray', alpha=0.3, linestyle="--", label="y=10x")
if y2:
    plt.plot([1/y2*border_lo, 1/y2*border_hi], [border_lo, border_hi], 'black', alpha=0.3, linestyle=":", label="y="+str(y2)+"x")

plt.plot([border_lo, lim], [lim, lim], 'black', alpha=1)
plt.plot([lim, lim], [border_lo, lim], 'black', alpha=1)
plt.fill_between([border_lo, lim], [lim, lim], [border_hi, border_hi], alpha=0.3, color='gray', zorder=0) #color='blue', label=str(timeouts_y) + " timeouts of LEFT")
plt.fill_between([lim, border_hi], [border_lo, border_lo], [lim, lim], alpha=0.3, color='gray', zorder=0) #, label=str(timeouts_x) + " timeouts of BOTTOM")

timeouts_x = 0
timeouts_y = 0
label_idx = 0


for dom in runtime_map_pairs_by_domain:
    runtime_maps = runtime_map_pairs_by_domain[dom]
    
    X = []
    Y = []
    keys = [i for i in runtime_maps[0]] + [i for i in runtime_maps[1]]
    for i in keys:
        
        if i not in runtime_maps[0] and i not in runtime_maps[1]:
            continue
        
        elif i not in runtime_maps[0]:
            Y += [runtime_maps[1][i]]
            X += [out]
            timeouts_x += 1
            print(str(i) + " : X timeout , Y " + str(runtime_maps[1][i]))
        elif i not in runtime_maps[1]:
            X += [runtime_maps[0][i]]
            Y += [out]
            timeouts_y += 1
            print(str(i) + " : X " + str(runtime_maps[0][i]) + ", Y timeout")
        else:
            X += [runtime_maps[0][i]]
            Y += [runtime_maps[1][i]]
            print(str(i) + " : X " + str(runtime_maps[0][i]) + ", Y " + str(runtime_maps[1][i]))

    marker = domainmarkers[label_idx%len(domainmarkers)] if domainmarkers else markers[label_idx%len(markers)]
    color = domaincolors[label_idx%len(domaincolors)] if domaincolors else colors[label_idx%len(colors)]
    label = domainlabels[label_idx%len(domainlabels)] if domainlabels else dom
    
    if label not in domain_to_style_map:
        domain_to_style_map[label] = (color, marker)
    
    plt.plot(X, Y, marker, color='None', label=label, alpha=1, markersize=msize, 
             markeredgecolor=color)
    # 'None' if marker in '.ov^<>8spP*hHXDd' else color
    label_idx += 1

if heading:
    plt.title(heading)
if xlabel:
    plt.xlabel(xlabel.replace(' ', '\ '))
else:    
    plt.xlabel(labels[0].replace(' ', '\ '))
if ylabel:
    plt.ylabel(ylabel.replace(' ', '\ '))
else:    
    plt.ylabel(labels[1].replace(' ', '\ '))

if not nolegend and not outfile_legend:    
    if legendright:
        plt.legend(bbox_to_anchor=(1.05+legend_offset_x, 0.5+legend_offset_y), loc='center left', edgecolor="black", ncol=legend_columns, labelspacing=legend_spacing, columnspacing=legend_spacing*2, handlelength=legend_spacing*2)
    elif legendbottom:
        plt.legend(bbox_to_anchor=(0.5+legend_offset_x, -0.27+legend_offset_y), loc='lower center', edgecolor="black", ncol=legend_columns, labelspacing=legend_spacing, columnspacing=legend_spacing*2, handlelength=legend_spacing*2)
    elif lloc:
        plt.legend(ncol=legend_columns, labelspacing=legend_spacing, columnspacing=legend_spacing*2, handlelength=legend_spacing*2, loc=lloc)
    else:
        plt.legend(ncol=legend_columns, labelspacing=legend_spacing, columnspacing=legend_spacing*2, handlelength=legend_spacing*2)

if topsymbols:
    for spec in topsymbols.split(','):
        [xstr, ystr, symbol, color] = spec.split(';')
        plt.plot([float(xstr)], [float(ystr)], symbol, color=color, zorder=10, markerfacecolor='None', clip_on=False)
if toptexts:
    for spec in toptexts.split(','):
        [xstr, ystr, text, color] = spec.split(';')
        plt.text(float(xstr), float(ystr), text, color=color, zorder=10, clip_on=False, ha='center', va='center')

plt.tight_layout()
if outfile:
    plt.savefig(outfile)
else:
    plt.show()



def flip_legend_labels(labels, plots, ncols):

    newlabels = [l for l in labels]
    newplots = [b for b in plots]
    r = 0
    c = 0

    for i in range(len(labels)):
        j = ncols * r + c
        newlabels[i] = labels[j]
        newplots[i] = plots[j]
        if ncols * (r+1) + c >= len(labels):
            r = 0
            c += 1
        else:
            r += 1
    
    return (newlabels, newplots)

if outfile_legend:
    
    plots = []
    domains = []
    for domain in domain_to_style_map:
        style = domain_to_style_map[domain]
        domains += [domain]
        plots += [matplotlib.lines.Line2D([], [], c=style[0], marker=style[1], lw=0, fillstyle='none', label=domain)]
    figlegend = plt.figure()
    ncols = 4
    (newlabels, newplots) = flip_legend_labels(domains, plots, ncols)
    print(newlabels)
    labelspacing = 0.2
    if labelspacing is not None:
        figlegend.legend(newplots, newlabels, 'center', ncol=ncols, edgecolor='#000000', labelspacing=labelspacing)
    else:
        figlegend.legend(newplots, newlabels, 'center', ncol=ncols, edgecolor='#000000')
    figlegend.set_edgecolor('b')
    figlegend.tight_layout()
    figlegend.savefig(outfile_legend, bbox_inches='tight')


