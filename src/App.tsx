import { invoke } from "@tauri-apps/api/tauri";
import { Grid, ButtonBase, Typography, styled, Button } from "@mui/material";
import "./App.css";

function App() {

  async function openApp(appName: string, title: string, url: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("open_app", { appName, title, url });
  }

  const apps = [
    {
      name: "gosti-library",
      title: "Library",
      url: "../resources/apps/gosti-library-dapp/index.html",
      logo: "../resources/apps/gosti-library-dapp/icon.webp",
    },
    {
      name: "gosti-marketplace",
      title: "Gosti Marketplace",
      url: "../resources/apps/gosti-marketplace-dapp/index.html",
      logo: "../resources/apps/gosti-marketplace-dapp/icon.webp",
    },
    {
      name: "gosti-publishing",
      title: "Publishing",
      url: "../resources/apps/gosti-marketplace-publishing-dapp/index.html",
      logo: "../resources/apps/gosti-marketplace-publishing-dapp/icon.webp",
    },
    {
      name: "gosti-profile",
      title: "Profile",
      url: "../resources/apps/gosti-profile-app/index.html",
      logo: "../resources/apps/gosti-profile-app/icon.webp",
    },
    {
      name: "chia-poker",
      title: "Chia Poker", 
      url: "../resources/apps/chia-poker/index.html", 
      logo: "../resources/apps/chia-poker/icon.webp"
    },
    {
      name: "gosti-streaming-tools",
      title: "Streaming Tools",
      url: "../resources/apps/gosti-streaming-tools/index.html",
      logo: "../resources/apps/gosti-streaming-tools/icon.webp",
    },
    {
      name: "about-gosti",
      title: "About",
      url: "../resources/apps/about/index.html",
      logo: "../resources/apps/about/icon.webp",
    },
  ];

  const ImageButton = styled(ButtonBase)(({ theme }) => ({
    position: 'relative',
    height: 200,
    [theme.breakpoints.down('sm')]: {
      width: '100% !important', // Overrides inline-style
      height: 100,
    },
    '&:hover, &.Mui-focusVisible': {
      zIndex: 1,
      '& .MuiImageBackdrop-root': {
        opacity: 0.15,
      },
      '& .MuiImageMarked-root': {
        opacity: 0,
      },
      '& .MuiTypography-root': {
        border: '4px solid currentColor',
        animation: 'border-rgb-2 9s infinite linear'
      },
    },
  }));
  
  const ImageSrc = styled('span')({
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundSize: 'cover',
    backgroundPosition: 'center 40%',
  });
  
  const Image = styled('span')(({ theme }) => ({
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: theme.palette.common.white,
  }));
  
  const ImageBackdrop = styled('span')(({ theme }) => ({
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundColor: theme.palette.common.black,
    opacity: 0.4,
    transition: theme.transitions.create('opacity'),
  }));
  
  const ImageMarked = styled('span')(({ theme }) => ({
    height: 3,
    width: 18,
    backgroundColor: theme.palette.common.white,
    position: 'absolute',
    bottom: -2,
    left: 'calc(50% - 9px)',
    transition: theme.transitions.create('opacity'),
  }));

  return (
    <div className="container">
      <img src="src/assets/GostiAnim.webp" width="100%" style={{maxWidth: '500px', alignSelf: 'center'}} />
      <h2>Installed Apps</h2>

      <div className="row">
        <Grid container spacing={1}>
          {apps.map((app) => (
            <Grid key={app.title} item xs={12} sm={6} md={4} lg={3}>
              <ImageButton
                focusRipple
                key={app.title}
                onClick={() => openApp(app.name, app.title, app.url)}
                style={{
                  width: '100%',
                }}
              >
                <ImageSrc style={{ backgroundImage: `url(${app.logo})` }} />
                <ImageBackdrop className="MuiImageBackdrop-root" />
                <Image>
                  <Typography
                    component="span"
                    variant="subtitle1"
                    color="inherit"
                    sx={{
                      position: 'relative',
                      p: 4,
                      pt: 2,
                      pb: (theme) => `calc(${theme.spacing(1)} + 6px)`,
                    }}
                  >
                    {app.title}
                    <ImageMarked className="MuiImageMarked-root" />
                  </Typography>
                </Image>
              </ImageButton>
            </Grid>
          ))}
        </Grid>
      </div>
    </div>
  );
}

export default App;
