# Layer 0: Datacenter & Physical Requirements

To run a 1,000-node fleet reliably, you can't just plug things into a power strip and hope for the best. This document outlines the physical environment needed to keep the Exile platform alive.

## 1. Power Management

A thousand machines pull a lot of juice. 
- **Dedicated Circuits:** Never share circuits with HVAC or general-purpose office outlets. You'll trip breakers.
- **UPS Protection:** All control-plane hardware (servers, switches, firewalls) *must* be behind a UPS. We don't care as much if a worker node loses power, but if the control plane goes down, the whole fleet is blind.
- **Monitoring:** If possible, use "Smart" PDUs. Being able to see how many Amps a specific rack is drawing from a dashboard is a lifesaver.

## 2. Cooling & Airflow

Heat is the silent killer of hash rates.
- **The Cold Aisle / Hot Aisle Model:** Fronts of the machines face each other (Cold Aisle); backs face each other (Hot Aisle). This prevents machines from sucking in each other's exhaust.
- **Don't Block the Back:** Cables are messy, but keep them clear of the exhaust fans. 
- **Sensors:** Put thermal sensors at the top and bottom of your racks. Heat rises, and the top-most machines are always the first to throttle.

## 3. Physical Security & Access

- **Lock the Rack:** Even if the room is secure, lock the individual racks containing the control plane gear.
- **Label Everything:** It sounds simple, but when a machine fails at 3 AM, you need to be able to find it instantly. Every node should have a physical ID that matches its digital ID in our registry.

## 4. Maintenance Space

- Keep at least 3 feet of clearance behind every rack. You *will* need to get back there to swap cables or power supplies.
- Have a "crash cart" (a monitor and keyboard on wheels) ready. Sometimes you just have to plug in locally to see why a node isn't booting.

---
*Remember: Hardware is the only part of the platform you can't fix with a Git commit. Treat it with respect.*
