<script lang="ts">
    import {
        faCloudSun,
        faMoon,
        faSun,
    } from '@fortawesome/free-solid-svg-icons'
    import { Button, Tooltip } from '@sveltestrap/sveltestrap'
    import { get } from 'svelte/store'
    import Fa from 'svelte-fa'

    import { currentTheme, setCurrentTheme } from 'theme'

    function toggle() {
        if (get(currentTheme) === 'auto') {
            setCurrentTheme('dark')
        } else if (get(currentTheme) === 'dark') {
            setCurrentTheme('light')
        } else {
            setCurrentTheme('auto')
        }
    }

    let computedAriaLabel = $derived(
        $currentTheme === 'dark' ? "Switch to light theme" :
        $currentTheme === 'light' ? "Switch to automatic theme" :
        "Switch to dark theme"
    )
</script>

<Button color="link" on:click={toggle} id="button" aria-label={computedAriaLabel}>
    {#if $currentTheme === 'dark'}
        <Fa fw icon={faMoon} />
    {:else if $currentTheme === 'light'}
        <Fa fw icon={faSun} />
    {:else}
        <Fa fw icon={faCloudSun} />
    {/if}
</Button>
<Tooltip target="button" animation placement="left" container="body">
    {#if $currentTheme === 'dark'}
        Dark theme
    {:else if $currentTheme === 'light'}
        Light theme
    {:else}
        Automatic theme
    {/if}
</Tooltip>
