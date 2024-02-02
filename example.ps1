[cmdletbinding()]
<#
    .SYNOPSIS
    This Script is used as an example for the rust crate psparser

    .DESCRIPTION
    The Parameters of this Script are supposed to be parsed by the rust crate psparser
#>
Param(
    [Parameter()][string]$DefaultValue='Test-Param1',
    [Parameter(Mandatory)]
    [int32]$Integer,
    [Parameter(
        Mandatory=$false
    )]
    [Boolean]
    $Boolean=$true,
    [Parameter()][switch]$Switch
)

process {
    Write-Host "This Script does absolutely nothing"
}