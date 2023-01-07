export percent_barplot, value_barplot

using Gadfly
using DataFrames
using ColorSchemes
using Compose

function percent_barplot(
    df::DataFrame, x::Symbol, y::Symbol, title::String;
    width=15cm, height=12cm,
    barspacing=2.0mm,
    palette=ColorSchemes.glasbey_bw_minc_20_minl_30_n256,
    xlabel::String=string(y), ylabel::String=string(x)
)
    set_default_plot_size(width, height)
    plot(
        df, y=x, x=y, 
        color=x, 
        Coord.cartesian(yflip=true), Scale.y_discrete,
        Geom.bar(orientation=:horizontal),
        Theme(bar_spacing=barspacing, key_position=:none),
        Scale.color_discrete_manual(palette...),
        Guide.title(title),
        Guide.xlabel(xlabel), Guide.ylabel(ylabel),
        Guide.annotation(
            compose(
                context(),
                text(
                    df[!, y] .- 0.03,
                    1:length(df[!, y]),
                    string.(round.(df[!, y] .* 100, digits=3), "%"),
                    [hright for x in df[!, y]],
                    [vcenter for x in df[!, y]],
                ),
                fontsize(2.5),
                fill("black")
            )
        )
    )
end

function value_barplot(
    df::DataFrame, x::Symbol, y::Symbol, title::String;
    width=15cm, height=8cm,
    barspacing=2.0mm,
    palette=ColorSchemes.seaborn_colorblind,
    xlabel::String=string(y), ylabel::String=string(x)
)
    set_default_plot_size(width, height)
    maxy = maximum(df[!, y])
    plot(
        df, y=x, x=y, 
        color=x, 
        Coord.cartesian(yflip=true), Scale.y_discrete,
        Geom.bar(orientation=:horizontal),
        Theme(bar_spacing=barspacing, key_position=:none),
        Scale.color_discrete_manual(palette...),
        Guide.title(title),
        Guide.xlabel(xlabel), Guide.ylabel(ylabel),
        Guide.annotation(
            compose(
                context(),
                text(
                    df[!, y] .- maxy * 0.03,
                    1:length(df[!, y]),
                    string.(round.(df[!, y], digits=5)),
                    [hright for x in df[!, y]],
                    [vcenter for x in df[!, y]],
                ),
                fontsize(2.5),
                fill("black")
            )
        )
    )
end